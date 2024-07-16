use crate::config::Settings;
use crate::markdown::convert_md_to_html;
use crate::post::Post;
use atom_syndication::{
    ContentBuilder, Entry, EntryBuilder, Feed, FeedBuilder, LinkBuilder, PersonBuilder,
    SourceBuilder, Text,
};
use std::fs::File;
use std::path::Path;

use anyhow::Result;

use chrono::{FixedOffset, Utc};

pub fn write_atom_for_posts(
    posts: &[Post],
    base: &str,
    author: &str,
    title: &str,
    path: &Path,
    settings: &Settings,
) -> Result<()> {
    let authors = vec![PersonBuilder::default().name(author.to_string()).build()];

    // Sort posts from older to newer (ASC front_matter date)
    let mut sorted_posts = posts.to_vec();
    sorted_posts.sort_by(|p1, p2| p1.front_matter.date.cmp(&p2.front_matter.date));

    let last_updated_at = match sorted_posts.last() {
        Some(post) => post.front_matter.date,
        None => Utc::now().with_timezone(&FixedOffset::west_opt(0).unwrap()),
    };

    let mut feed: Feed = FeedBuilder::default()
        .links(vec![
            LinkBuilder::default()
                .rel("alternate".to_string())
                .href(base.to_string())
                .build(),
            LinkBuilder::default()
                .rel("license".to_string())
                .href("https://creativecommons.org/licenses/by/4.0/".to_string())
                .build(),
        ])
        .authors(authors.clone())
        .rights(Some(Text::plain("CC-By Licence")))
        .base(Some(base.into()))
        .lang(Some("fr-FR".into()))
        .id(format!("{}/", base))
        .title(title)
        .updated(last_updated_at)
        .build();

    // For every Post, write the HTML to the correct directory
    for post in sorted_posts {
        let html_content = convert_md_to_html(&post.content, settings, Some(&post.path[..]));
        let entry: Entry = EntryBuilder::default()
            .title(&post.front_matter.title[..])
            .id(format!("{}/{}", base, post.url_path_encoded))
            .updated(post.front_matter.date)
            .authors(authors.clone())
            .contributors(authors.clone())
            .links(vec![LinkBuilder::default()
                .rel("alternate".to_string())
                .href(format!("{}/{}", base, post.url_path_encoded))
                .build()])
            .published(Some(post.front_matter.date))
            .summary(Some(Text::plain(
                post.excerpt.as_ref().unwrap_or(&"".to_string()),
            )))
            .rights(Some(Text::plain("CC-By Licence")))
            .content(Some(
                ContentBuilder::default()
                    .value(Some(html_content.clone()))
                    .content_type(Some(String::from("html")))
                    .build(),
            ))
            .source(Some(
                SourceBuilder::default()
                    .title(&post.front_matter.title[..])
                    .id(format!("{}/{}", base, post.url_path_encoded))
                    .updated(post.front_matter.date)
                    .build(),
            ))
            .build();

        feed.entries.push(entry);
    }

    let buffer = File::create(path)?;

    feed.write_to(buffer).unwrap();

    Ok(())
}
