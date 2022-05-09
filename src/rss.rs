use crate::post::Post;
use atom_syndication::{
    ContentBuilder, Entry, EntryBuilder, Feed, FeedBuilder, LinkBuilder, PersonBuilder,
    SourceBuilder, Text,
};
use std::fs::File;
use std::path::Path;

pub fn write_atom_for_posts(posts: &[Post], base: &str, author: &str, path: &Path) {
    let authors = vec![PersonBuilder::default().name(author).build()];

    let mut feed: Feed = FeedBuilder::default()
        .links(vec![
            LinkBuilder::default().rel("self").href(base).build(),
            LinkBuilder::default()
                .rel("license")
                .href("https://creativecommons.org/licenses/by/4.0/")
                .build(),
        ])
        .authors(authors.clone())
        .rights(Some(Text::plain("CC-By Licence")))
        .base(Some(base.into()))
        .lang(Some("fr_FR".into()))
        .build();

    // For every Post, write the HTML to the correct directory
    for post in posts {
        let entry: Entry = EntryBuilder::default()
            .title(&post.front_matter.title[..])
            .id(&format!("{}/{}", base, post.url_path)[..])
            .updated(post.front_matter.date)
            .authors(authors.clone())
            .contributors(authors.clone())
            .links(vec![LinkBuilder::default()
                .rel("self")
                .href(&format!("{}/{}", base, post.url_path)[..])
                .build()])
            .published(Some(post.front_matter.date))
            .summary(Some(Text::plain(
                post.excerpt.as_ref().unwrap_or(&"".to_string()),
            )))
            .rights(Some(Text::plain("CC-By Licence")))
            .content(Some(
                ContentBuilder::default()
                    .value(Some(post.content.clone()))
                    .build(),
            ))
            .source(Some(
                SourceBuilder::default()
                    .title(&post.front_matter.title[..])
                    .id(&format!("{}/{}", base, post.url_path)[..])
                    .updated(post.front_matter.date)
                    .build(),
            ))
            .build();

        feed.entries.push(entry);
    }

    let buffer = File::create(path);

    // @TODO: manage errors properly with a Result type
    match buffer {
        Ok(b) => {
            feed.write_to(b).unwrap();
        }
        _ => println!("Can't write"),
    }
}
