use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::fs;
use walkdir::WalkDir;

type FilePath = String;

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

pub fn get_files_for_directory(directory: &str) -> Vec<FilePath> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .map(|entry| String::from(entry.path().to_string_lossy()))
        .collect()
}

pub fn list_directory(directory: &str) {
    let files_to_parse: Vec<FilePath> = get_files_for_directory(directory);

    let posts_contents: Vec<PostContent> = files_to_parse
        .into_iter()
        .filter_map(|file_path| parse_file(&file_path))
        .collect();

    log::info!("{:?}", posts_contents);
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
