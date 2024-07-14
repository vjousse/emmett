use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S%z";

mod custom_date_format {
    use crate::post::FORMAT;
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer, Serializer};

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

mod optional_custom_date_format {
    use crate::post::FORMAT;
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(
        date: &Option<DateTime<FixedOffset>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                let s = format!("{}", d.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parsed_date = DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

        Ok(Some(parsed_date))
    }
}

mod parse_tags {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(tags: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match tags {
            Some(t) => serializer.serialize_str(&t.join(",")),
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer);

        let v = match s {
            Ok(content) => Some(
                content
                    .split(',')
                    .map(|t| t.trim().to_lowercase().to_string())
                    .collect(),
            ),
            Err(_) => None,
        };

        Ok(v)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
}

#[derive(Clone, Deserialize, Debug, Serialize, Eq, Ord, PartialEq, PartialOrd)]
// Used by gray_matter engine to parse the Front Matter content
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    #[serde(with = "custom_date_format")]
    pub date: DateTime<FixedOffset>,
    pub status: Option<PostStatus>,
    pub category: Option<String>,
    // Be sure that we keep an Option if the field is missing
    // See https://stackoverflow.com/questions/44301748/how-can-i-deserialize-an-optional-field-with-custom-functions-using-serde
    #[serde(default)]
    #[serde(with = "parse_tags")]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    #[serde(with = "optional_custom_date_format")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub toc: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Post {
    pub front_matter: FrontMatter,
    pub date_rfc3339: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub path: String,
    pub url_path: String,
    pub url_path_encoded: String,
    pub ancestor_directories_paths: Vec<String>,
    pub ancestor_directories_names: Vec<String>,
}

impl Post {
    pub fn new(
        front_matter: FrontMatter,
        excerpt: Option<String>,
        content: String,
        path: String,
        url_path: String,
        url_path_encoded: String,
        ancestor_directories_paths: Vec<String>,
        ancestor_directories_names: Vec<String>,
    ) -> Self {
        let rfc_date = front_matter.date.to_rfc3339();
        Post {
            content,
            front_matter,
            excerpt,
            path,
            url_path,
            url_path_encoded,
            date_rfc3339: rfc_date,
            ancestor_directories_paths,
            ancestor_directories_names,
        }
    }
}
