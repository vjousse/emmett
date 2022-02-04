use gray_matter::engine::YAML;
use gray_matter::Matter;
use lazy_static::lazy_static;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
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
pub struct PostContent {
    pub front_matter: Option<FrontMatter>,
    pub excerpt: Option<String>,
    pub content: String,
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

pub fn list_directory(directory: &str) {
    // Get the list of files
    let files_to_parse: Vec<FilePath> = get_files_for_directory(directory);

    // Read content of every file, get the front_matter content
    let posts_contents: Vec<PostContent> = files_to_parse
        .into_iter()
        .filter_map(|file_path| parse_file(&file_path))
        .collect();

    for post in &posts_contents {
        let html_content = convert_md_to_html(&post.content);
        //log::info!("{}", html_content);

        let mut context = Context::new();

        if let Some(front_matter) = &post.front_matter {
            log::info!("{:?}", &front_matter.title);
            context.insert("title", &front_matter.title);
            context.insert("date", &front_matter.date);

            context.insert("post_content", &html_content);

            if let Some(html) = render_template_to_html(context, "blog/post.html") {
                let mut f = File::create(format!("output/{}.html", &front_matter.slug))
                    .expect("Unable to create file");
                f.write_all(html.as_bytes()).expect("Unable to write data");
            };
        };
    }

    //log::info!("{:?}", posts_contents);
}

pub fn parse_file(file_path: &str) -> Option<PostContent> {
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let post_content = parse_content(content);
            Some(post_content)
        }
        Err(e) => {
            log::error!("Error for {}: {}", &file_path, e);
            None
        }
    }
}

pub fn parse_content(content: String) -> PostContent {
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

    PostContent {
        content: result.content,
        front_matter,
        excerpt: result.excerpt,
    }
}
