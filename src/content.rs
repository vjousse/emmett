use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::fs;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
// Used by gray_matter engine to parse the Front Matter content
pub struct FrontMatter {
    title: String,
    slug: String,
    date: String,
}

#[derive(Debug)]
pub struct PostContent {
    front_matter: Option<FrontMatter>,
    excerpt: Option<String>,
    content: String,
}

pub fn list_directory(directory: &str) {
    let post_contents: Vec<PostContent> = WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .map(|entry| String::from(entry.path().to_string_lossy()))
        .filter_map(|file_path| parse_file(&file_path))
        .collect();

    log::info!("{:?}", post_contents);
}

pub fn parse_file(file_path: &String) -> Option<PostContent> {
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
