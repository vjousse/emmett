use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::fs;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    slug: String,
    date: String,
}

pub fn list_directory(directory: &str) {
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let file_path = String::from(entry.path().to_string_lossy());

        log::info!("Reading {} content", file_path);
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                parse_content(content);
            }
            Err(e) => log::error!("Error for {}: {}", &file_path, e),
        };
    }
}

pub fn parse_content(content: String) -> String {
    let mut matter = Matter::<YAML>::new();
    matter.excerpt_delimiter = Some("<!-- TEASER_END -->".to_owned());
    let result = matter.parse(&content[..]);
    match result.data {
        Some(data) => {
            //let front_matter: FrontMatter = data.deserialize().unwrap();
            match data.deserialize::<FrontMatter>() {
                Ok(front_matter) => log::info!("{:?}", front_matter),
                Err(e) => log::error!("Unable to read front matter. Is it a valid YAML format? Check that your title doesn't contain the ':' character."),
            }
        }
        None => log::info!("No data found in front matter"),
    }
    log::info!("{:?}", result.excerpt);

    result.content
}
