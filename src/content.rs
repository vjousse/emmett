use self::cmark::{Event, Options, Parser, Tag};
use crate::codeblock::{CodeBlock, FenceSettings};
use crate::config::Settings;
use crate::post::{FrontMatter, Post, PostStatus};
use crate::rss::write_atom_for_posts;
use crate::site::Site;
use anyhow::Result;
use form_urlencoded::byte_serialize;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark as cmark;
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

pub fn create_content(site: &Site) -> Result<()> {
    // Get the list of files
    let files_to_parse: Vec<FilePath> = get_files_for_directory(&site.settings.posts_path);

    // Convert the list of files to Post instances
    // @TODO: Don't load all the md/html in memory but read and write post per post
    let posts: Vec<Post> = get_posts(
        &files_to_parse,
        &site.settings.posts_path,
        &site.settings.blog_prefix_path,
    );

    // Write posts onto disk
    write_posts_html(&posts, site);

    // Sort the posts per indexes fr, en and so on
    let indexes_to_create: HashMap<String, Vec<&Post>> = get_posts_per_indexes(&posts, site);

    // Write down the list of posts in the index directory fr/, en/, …
    write_indexes_html(indexes_to_create, site);

    // Get the list of pages
    let pages_files: Vec<FilePath> = get_files_for_directory(&site.settings.pages_path);

    // Well let's say that a page and a post are the sames for now, we may
    // want to be more generic later on
    let pages: Vec<Post> = get_posts(&pages_files, &site.settings.pages_path, "");

    write_posts_html(&pages, site);

    write_atom_for_posts(
        &posts,
        "https://vincent.jousse.org",
        "Vincent Jousse",
        "Vince's",
        Path::new(&format!("{}/atom.xml", &site.settings.output_path)[..]),
    )?;

    Ok(())
}

pub fn convert_md_to_html(md_content: &str, settings: &Settings, path: Option<&str>) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    //let parser = Parser::new_ext(md_content, options);

    let mut events = Vec::new();
    let mut code_block: Option<CodeBlock> = None;

    for (event, mut _range) in Parser::new_ext(md_content, options).into_offset_iter() {
        match event {
            Event::Text(text) => {
                if let Some(ref mut code_block) = code_block {
                    let html;
                    html = code_block.highlight(&text);
                    events.push(Event::Html(html.into()));
                } else {
                    events.push(Event::Text(text));
                    continue;
                }
            }
            Event::Start(Tag::CodeBlock(ref kind)) => {
                let fence = match kind {
                    cmark::CodeBlockKind::Fenced(fence_info) => FenceSettings::new(fence_info),
                    _ => FenceSettings::new(""),
                };
                let (block, begin) = CodeBlock::new(fence, settings, path);
                code_block = Some(block);
                events.push(Event::Html(begin.into()));
            }
            Event::End(Tag::CodeBlock(_)) => {
                // reset highlight and close the code block
                code_block = None;
                events.push(Event::Html("</code></pre>\n".into()));
            }
            _ => events.push(event),
        }
    }

    // We remove all the empty things we might have pushed before so we don't get some random \n
    events = events
        .into_iter()
        .filter(|e| match e {
            Event::Text(text) | Event::Html(text) => !text.is_empty(),
            _ => true,
        })
        .collect();

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(md_content.len() * 3 / 2);
    //html::push_html(&mut html_output, parser);
    cmark::html::push_html(&mut html_output, events.into_iter());

    html_output
}

pub fn render_template_to_html(
    context: Context,
    template_path: &str,
    tera: &Tera,
) -> Option<String> {
    match tera.render(template_path, &context) {
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

// Read content of every file, and create a Post instance
// Only keep posts that don't have the draft status
pub fn get_posts(files_to_parse: &[FilePath], posts_path: &str, prefix_path: &str) -> Vec<Post> {
    files_to_parse
        .iter()
        .filter_map(|file_path| parse_file(file_path, posts_path, prefix_path))
        // Don't publish posts that are still drafts
        .filter(|post| match &post.front_matter.status {
            Some(status) => *status != PostStatus::Draft,
            None => true,
        })
        .collect()
}

pub fn write_posts_html(posts: &[Post], site: &Site) {
    // For every Post, write the HTML to the correct directory
    for post in posts {
        let html_content = convert_md_to_html(&post.content, &site.settings, Some(&post.path[..]));

        let mut context = Context::new();

        let front_matter = &post.front_matter;
        context.insert("title", &front_matter.title);
        context.insert("date", &front_matter.date.to_rfc3339());

        context.insert("post_content", &html_content);
        context.insert("post_url_path", &post.url_path);

        if let Some(html) = render_template_to_html(context, "blog/post.html", &site.tera) {
            write_post_html(&html, post, &site.settings.output_path);
        };
    }
}

pub fn get_posts_per_indexes<'a>(posts: &'a [Post], site: &Site) -> HashMap<String, Vec<&'a Post>> {
    let mut indexes_to_create: HashMap<String, Vec<&Post>> = HashMap::new();

    // For every Post, let's see if it's part of a prefix we want an index for
    // If it's the case, generate an HashMap of posts for each prefix
    for post in posts {
        if let Ok(path) = Path::new(&post.path).strip_prefix(&site.settings.posts_path) {
            let mut components = path.components();

            if let Some(Component::Normal(first_component)) = components.next() {
                let first_component_str = first_component.to_str().unwrap_or("").to_owned();
                if site
                    .settings
                    .create_index_for
                    .contains(&first_component_str)
                {
                    let key = first_component_str.clone();
                    let posts = indexes_to_create.entry(key).or_insert_with(Vec::new);
                    posts.push(post);
                }
            }
        }
    }

    indexes_to_create
}

pub fn write_indexes_html(indexes_to_create: HashMap<String, Vec<&Post>>, site: &Site) {
    for (index, mut posts) in indexes_to_create {
        let mut context = Context::new();
        let _ = &posts.sort_by(|p1, p2| p2.front_matter.date.cmp(&p1.front_matter.date));
        context.insert("posts", &posts);
        context.insert("title", &format!("Articles - {}", &index)[..]);
        if let Some(html) = render_template_to_html(context, "blog/list.html", &site.tera) {
            write_html(
                &html,
                &format!(
                    "{}/{}/{}",
                    site.settings.output_path, site.settings.blog_prefix_path, index
                )[..],
            );
        };
    }
}

pub fn get_output_directory_for_post(output_directory: String, post: &Post) -> String {
    let mut out_path = PathBuf::from(&output_directory);

    if post.url_path != "/" {
        out_path.push(&post.url_path);
    }

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
    let post_output_directory = get_output_directory_for_post(output_directory.to_owned(), post);

    write_html(post_html, &post_output_directory);
}

pub fn parse_file(file_path: &str, input_directory: &str, blog_prefix_path: &str) -> Option<Post> {
    match fs::read_to_string(&file_path) {
        Ok(content) => parse_post(content, file_path, input_directory, blog_prefix_path),
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
) -> Option<Post> {
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

    let path_url = extract_path_url_for_post(
        &front_matter,
        file_path,
        input_directory,
        blog_prefix_path,
        false,
    );

    let path_url_encoded = extract_path_url_for_post(
        &front_matter,
        file_path,
        input_directory,
        blog_prefix_path,
        true,
    );

    //byte_serialize(post.url_path.as_bytes()).collect::<String>()

    front_matter.map(|fm| {
        Post::new(
            fm,
            result.excerpt,
            result.content,
            file_path.to_owned(),
            path_url,
            path_url_encoded,
        )
    })
}

pub fn extract_path_url_for_post(
    front_matter: &Option<FrontMatter>,
    file_path: &str,
    input_directory: &str,
    blog_prefix_path: &str,
    encoded: bool,
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
        Some(front_matter) => {
            if encoded {
                url_path.push(byte_serialize(front_matter.slug.as_bytes()).collect::<String>())
            } else {
                url_path.push(&front_matter.slug)
            }
        }
        None => (),
    };

    let url_path_str = url_path.to_str().unwrap_or(file_path).to_owned();

    output_path.push(if !url_path_str.ends_with('/') {
        format!("{}/", url_path_str)
    } else {
        url_path_str
    });

    output_path.to_str().unwrap_or(file_path).to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_front_matter() -> FrontMatter {
        let raw = r#"{"title": "Mes dernières découvertes", "slug": "mes-dernières-decouvertes-1", "date": "2019-09-04 17:20:00+01:00"}"#;
        let front_matter: FrontMatter = serde_json::from_str(raw).expect("Couldn't derserialize");
        front_matter
    }

    fn get_post() -> Post {
        let front_matter = get_front_matter();

        Post::new(
            front_matter,
            Some("Excerpt".to_owned()),
            "Content test".to_owned(),
            "content/fr/2019-09-04-mes-dernieres-decouvertes-1.md".to_owned(),
            "fr/2019-09-04-mes-dernieres-decouvertes-1/".to_owned(),
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
            extract_path_url_for_post(
                &Some(post.front_matter),
                &path,
                input_directory,
                blog_prefix_path,
                false
            ),
            "blog/fr/mes-dernières-decouvertes-1/"
        );
    }

    #[test]
    fn test_extract_path_url_encoded() {
        let post = get_post();

        let input_directory = "content";
        let blog_prefix_path = "blog";
        let path = "content/fr/2019-09-04-mes-dernieres-decouvertes-1.md";
        assert_eq!(
            extract_path_url_for_post(
                &Some(post.front_matter),
                &path,
                input_directory,
                blog_prefix_path,
                true
            ),
            "blog/fr/mes-derni%C3%A8res-decouvertes-1/"
        );
    }

    #[test]
    fn test_extract_path_url_with_input_trailing_slash() {
        let post = get_post();

        let input_directory = "content/";
        let blog_prefix_path = "blog";
        let path = "content/fr/2019-09-04-mes-dernieres-decouvertes-1.md";
        assert_eq!(
            extract_path_url_for_post(
                &Some(post.front_matter),
                &path,
                input_directory,
                blog_prefix_path,
                false
            ),
            "blog/fr/mes-dernières-decouvertes-1/"
        );
    }

    #[test]
    fn test_extract_path_url_with_input_slash() {
        let post = get_post();

        let blog_prefix_path = "blog";
        let input_directory = "content/";
        let path = "/content/fr/2019-09-04-mes-dernieres-decouvertes-1.md";
        assert_eq!(
            extract_path_url_for_post(
                &Some(post.front_matter),
                &path,
                input_directory,
                blog_prefix_path,
                false
            ),
            "blog/fr/mes-dernières-decouvertes-1/"
        );
    }
}
