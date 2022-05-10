use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

mod custom_date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%z";

    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        log::debug!("{:?}", s);
        let parsed_date = DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom);

        if parsed_date.is_err() {
            log::error!("{:?}", parsed_date);
        }

        parsed_date
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
}

#[derive(Deserialize, Debug, Serialize, Eq, Ord, PartialEq, PartialOrd)]
// Used by gray_matter engine to parse the Front Matter content
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    #[serde(with = "custom_date_format")]
    pub date: DateTime<FixedOffset>,
    pub status: Option<PostStatus>,
}

#[derive(Debug, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Post {
    pub front_matter: FrontMatter,
    pub excerpt: Option<String>,
    pub content: String,
    pub path: String,
    pub url_path: String,
    pub url_path_encoded: String,
}

impl Post {
    pub fn new(
        front_matter: FrontMatter,
        excerpt: Option<String>,
        content: String,
        path: String,
        url_path: String,
        url_path_encoded: String,
    ) -> Self {
        Post {
            content,
            front_matter,
            excerpt,
            path,
            url_path,
            url_path_encoded,
        }
    }
}
