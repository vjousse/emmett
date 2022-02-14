use gray_matter::engine::YAML;
use gray_matter::Matter;
use lazy_static::lazy_static;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Component;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};
use walkdir::WalkDir;

type FilePath = String;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

#[derive(Deserialize, Debug, Serialize)]
// Used by gray_matter engine to parse the Front Matter content
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    pub date: String,
}

#[derive(Debug, Serialize)]
pub struct Post {
    pub front_matter: Option<FrontMatter>,
    pub excerpt: Option<String>,
    pub content: String,
    pub path: String,
    pub url_path: String,
}

impl Post {
    fn new(
        front_matter: Option<FrontMatter>,
        excerpt: Option<String>,
        content: String,
        path: String,
        url_path: String,
    ) -> Self {
        Post {
            content,
            front_matter,
            excerpt,
            path,
            url_path,
        }
    }
}

pub fn convert_md_to_html(md_content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md_content, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(md_content.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn render_template_to_html(context: Context, template_path: &str) -> Option<String> {
    match TEMPLATES.render(template_path, &context) {
        Ok(s) => Some(s),
        Err(e) => {
            println!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                println!("Reason: {}", e);
                cause = e.source();
            }
            None
        }
    }
}

pub fn get_files_for_directory(directory: &str) -> Vec<FilePath> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .map(|entry| String::from(entry.path().to_string_lossy()))
        .collect()
}

pub fn create_content(
    input_directory: &str,
    output_directory: &str,
    blog_prefix_path: &str,
    create_index_for: Vec<String>,
) {
    // Get the list of files
    let files_to_parse: Vec<FilePath> = get_files_for_directory(input_directory);

    // Read content of every file, and create a Post instance
    let posts_contents: Vec<Post> = files_to_parse
        .into_iter()
        .filter_map(|file_path| parse_file(&file_path, input_directory, blog_prefix_path))
        .collect();

    // For every Post, write the HTML to the correct directory
    for post in &posts_contents {
        let html_content = convert_md_to_html(&post.content);

        let mut context = Context::new();

        if let Some(front_matter) = &post.front_matter {
            log::debug!("{:?}", &front_matter.title);
            context.insert("title", &front_matter.title);
            context.insert("date", &front_matter.date);

            context.insert("post_content", &html_content);

            if let Some(html) = render_template_to_html(context, "blog/post.html") {
                write_post_html(&html, &post, output_directory);
            };
        };
    }

    let mut indexes_to_create: HashMap<String, Vec<&Post>> = HashMap::new();

    // For every Post, let's see if it's part of a prefix we want an index for
    // If it's the case, generate an HashMap of posts for each prefix
    for post in &posts_contents {
        if let Ok(path) = Path::new(&post.path).strip_prefix(input_directory) {
            let mut components = path.components();

            if let Some(Component::Normal(first_component)) = components.next() {
                let first_component_str = first_component.to_str().unwrap_or("").to_owned();
                if create_index_for.contains(&first_component_str) {
                    let key = first_component_str.clone();
                    let posts = indexes_to_create.entry(key).or_insert(vec![]);
                    posts.push(&post);
                }
            }
        }
    }

    log::info!("{:?}", indexes_to_create);

    for (index, posts) in &indexes_to_create {
        println!("=> {}", index);

        let mut context = Context::new();
        context.insert("posts", &posts);
        context.insert("title", &format!("Index of {}", &index)[..]);
        if let Some(html) = render_template_to_html(context, "blog/list.html") {
            write_html(&html, &format!("{}/{}", output_directory, index)[..]);
        };
    }
}

pub fn get_output_directory_for_post(output_directory: String, post: &Post) -> String {
    let mut out_path = PathBuf::from(&output_directory);

    out_path.push(&post.url_path);

    out_path.to_str().unwrap_or(&output_directory).to_owned()
}

pub fn write_html(post_html: &str, output_directory: &str) {
    fs::create_dir_all(&output_directory)
        .expect(&format!("Unable to create output directory {}", &output_directory)[..]);

    let mut f =
        File::create(format!("{}/index.html", &output_directory)).expect("Unable to create file");
    f.write_all(post_html.as_bytes())
        .expect("Unable to write data");
}

pub fn write_post_html(post_html: &str, post: &Post, output_directory: &str) {
    let post_output_directory = get_output_directory_for_post(output_directory.to_owned(), &post);

    write_html(post_html, &post_output_directory);
}

pub fn parse_file(file_path: &str, input_directory: &str, blog_prefix_path: &str) -> Option<Post> {
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let post = parse_post(content, &file_path, input_directory, blog_prefix_path);
            Some(post)
        }
        Err(e) => {
            log::error!("Error for {}: {}", &file_path, e);
            None
        }
    }
}

pub fn parse_post(
    content: String,
    file_path: &str,
    input_directory: &str,
    blog_prefix_path: &str,
) -> Post {
    let mut matter = Matter::<YAML>::new();
    matter.excerpt_delimiter = Some("<!-- TEASER_END -->".to_owned());
    let result = matter.parse(content.trim());
    let front_matter: Option<FrontMatter> = match result.data {
        Some(data) => match data.deserialize::<FrontMatter>() {
            Ok(front_matter) => Some(front_matter),
            Err(e) => {
                log::error!("Unable to read front matter. Is it a valid YAML format? Check for example that your title doesn't contain the ':' character.");
                log::error!("{}", e);
                None
            }
        },
        None => {
            log::info!("No data found in front matter {:?}", content);
            None
        }
    };

    let path_url =
        extract_path_url_for_post(&front_matter, &file_path, input_directory, blog_prefix_path);

    Post::new(
        front_matter,
        result.excerpt,
        result.content,
        file_path.to_owned(),
        path_url,
    )
}

pub fn extract_path_url_for_post(
    front_matter: &Option<FrontMatter>,
    file_path: &str,
    input_directory: &str,
    blog_prefix_path: &str,
) -> String {
    let mut url_path = PathBuf::from(&file_path);
    let mut output_path = PathBuf::from(blog_prefix_path);

    // Remove the file name
    // content/fr/test.md => content/fr
    url_path.pop();

    // Remove / prefix if any
    if let Ok(new_path) = url_path.strip_prefix("/") {
        url_path = new_path.to_path_buf();
    }

    // Remove the input directory prefix where the files are stored
    // content/fr => fr
    if let Ok(new_path) = url_path.strip_prefix(input_directory) {
        url_path = new_path.to_path_buf();
    }

    match front_matter {
        Some(front_matter) => url_path.push(&front_matter.slug),
        None => (),
    };

    let url_path_str = url_path.to_str().unwrap_or(&file_path).to_owned();

    output_path.push(if !url_path_str.ends_with("/") {
        format!("{}/", url_path_str)
    } else {
        url_path_str
    });

    output_path.to_str().unwrap_or(&file_path).to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_front_matter() -> FrontMatter {
        FrontMatter {
            title: "Mes dernières découvertes".to_owned(),
            slug: "mes-dernieres-decouvertes-1".to_owned(),
            date: "2019-09-04 17:20:00+01:00".to_owned(),
        }
    }

    fn get_post() -> Post {
        let front_matter = get_front_matter();

        Post::new(
            Some(front_matter),
            Some("Excerpt".to_owned()),
            "Content test".to_owned(),
            "content/fr/2019-09-04-mes-dernieres-decouvertes-1.md".to_owned(),
            "fr/2019-09-04-mes-dernieres-decouvertes-1/".to_owned(),
        )
    }

    #[test]
    fn test_extract_path_url() {
        let post = get_post();

        let input_directory = "content";
        let blog_prefix_path = "blog";
        let path = "content/fr/2019-09-04-mes-dernieres-decouvertes-1.md";
        assert_eq!(
            extract_path_url_for_post(&post.front_matter, &path, input_directory, blog_prefix_path),
            "blog/fr/mes-dernieres-decouvertes-1/"
        );
    }

    #[test]
    fn test_extract_path_url_with_input_trailing_slash() {
        let post = get_post();

        let input_directory = "content/";
        let blog_prefix_path = "blog";
        let path = "content/fr/2019-09-04-mes-dernieres-decouvertes-1.md";
        assert_eq!(
            extract_path_url_for_post(&post.front_matter, &path, input_directory, blog_prefix_path),
            "blog/fr/mes-dernieres-decouvertes-1/"
        );
    }

    #[test]
    fn test_extract_path_url_with_input_slash() {
        let post = get_post();

        let blog_prefix_path = "blog";
        let input_directory = "content/";
        let path = "/content/fr/2019-09-04-mes-dernieres-decouvertes-1.md";
        assert_eq!(
            extract_path_url_for_post(&post.front_matter, &path, input_directory, blog_prefix_path),
            "blog/fr/mes-dernieres-decouvertes-1/"
        );
    }
}
