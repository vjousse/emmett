use gray_matter::engine::YAML;
use gray_matter::Matter;
use lazy_static::lazy_static;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
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

#[derive(Deserialize, Debug)]
// Used by gray_matter engine to parse the Front Matter content
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    pub date: String,
}

#[derive(Debug)]
pub struct Post {
    pub front_matter: Option<FrontMatter>,
    pub excerpt: Option<String>,
    pub content: String,
    pub path: String,
}

impl Post {
    fn new(
        front_matter: Option<FrontMatter>,
        excerpt: Option<String>,
        content: String,
        path: String,
    ) -> Self {
        Post {
            content,
            front_matter,
            excerpt,
            path,
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

pub fn parse_directory(input_dir: &str, output_dir: &str, blog_prefix_path: &str) {
    // Get the list of files
    let files_to_parse: Vec<FilePath> = get_files_for_directory(input_dir);

    // Read content of every file, get the front_matter content
    let posts_contents: Vec<Post> = files_to_parse
        .into_iter()
        .filter_map(|file_path| parse_file(&file_path))
        .collect();

    for post in &posts_contents {
        let html_content = convert_md_to_html(&post.content);

        let mut context = Context::new();

        if let Some(front_matter) = &post.front_matter {
            log::debug!("{:?}", &front_matter.title);
            context.insert("title", &front_matter.title);
            context.insert("date", &front_matter.date);

            context.insert("post_content", &html_content);

            if let Some(html) = render_template_to_html(context, "blog/post.html") {
                write_post_html(&html, input_dir, &post, output_dir, blog_prefix_path);
            };
        };
    }
}

pub fn get_output_directory_for_post(
    output_directory: String,
    input_directory: String,
    post: &Post,
    blog_prefix_path: String,
) -> String {
    let mut out_path = PathBuf::from(&output_directory);
    let mut path = PathBuf::from(&post.path);

    // Remove the file name
    // content/fr/test.md => content/fr
    path.pop();

    // Remove / prefix if any
    if let Ok(new_path) = path.strip_prefix("/") {
        path = new_path.to_path_buf();
    }

    // Remove the input directory prefix where the files are stored
    // content/fr => fr
    if let Ok(new_path) = path.strip_prefix(&input_directory) {
        path = new_path.to_path_buf();
    }

    // Append the initial directory schema to the the ouptput directory
    // content/fr => output/fr
    out_path.push(blog_prefix_path);
    out_path.push(path);

    match &post.front_matter {
        Some(front_matter) => out_path.push(&front_matter.slug),
        None => (),
    };

    out_path.to_str().unwrap_or(&output_directory).to_owned()
}

pub fn write_post_html(
    post_html: &str,
    input_directory: &str,
    post: &Post,
    output_directory: &str,
    blog_prefix_path: &str,
) {
    let post_output_directory = get_output_directory_for_post(
        output_directory.to_owned(),
        input_directory.to_owned(),
        &post,
        blog_prefix_path.to_owned(),
    );

    fs::create_dir_all(&post_output_directory).expect(
        &format!(
            "Unable to create output directory {}",
            &post_output_directory
        )[..],
    );

    let mut f = File::create(format!("{}/index.html", &post_output_directory))
        .expect("Unable to create file");
    f.write_all(post_html.as_bytes())
        .expect("Unable to write data");
}

pub fn parse_file(file_path: &str) -> Option<Post> {
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let post = parse_post(content, &file_path);
            Some(post)
        }
        Err(e) => {
            log::error!("Error for {}: {}", &file_path, e);
            None
        }
    }
}

pub fn parse_post(content: String, file_path: &str) -> Post {
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

    Post::new(
        front_matter,
        result.excerpt,
        result.content,
        file_path.to_owned(),
    )
}
