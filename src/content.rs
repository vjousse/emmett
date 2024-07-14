use self::cmark::{Event, Options, Parser, Tag, TagEnd};
use crate::codeblock::{CodeBlock, FenceSettings};
use crate::config::Settings;
use crate::post::{FrontMatter, Post, PostStatus};
use crate::rss::write_atom_for_posts;
use crate::site::Site;
use crate::sitemap::write_sitemap_for_posts;
use anyhow::Result;
use form_urlencoded::byte_serialize;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use once_cell::sync::Lazy;
use pulldown_cmark::{self as cmark};
use pulldown_cmark_toc::TableOfContents;
use regex::Regex;
use slug::slugify;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::slice::Iter;
use strip_markdown::strip_markdown;
use tera::{Context, Tera};
use walkdir::WalkDir;

type FilePath = String;

/// Represents a heading.
#[derive(Debug, Clone)]
pub struct CustomHeading<'a> {
    /// The Markdown events between the heading tags.
    events: Vec<Event<'a>>,
    /// The heading level.
    tag: Tag<'a>,
}

impl CustomHeading<'_> {
    /// The raw events contained between the heading tags.
    pub fn events(&self) -> Iter<Event> {
        self.events.iter()
    }

    /// The heading level.
    pub fn tag(&self) -> Tag {
        self.tag.clone()
    }

    /// The heading text with all Markdown code stripped out.
    ///
    /// The output of this this function can be used to generate an anchor.
    pub fn text(&self) -> String {
        let mut buf = String::new();
        for event in self.events() {
            if let Event::Text(s) | Event::Code(s) = event {
                buf.push_str(s);
            }
        }

        buf
    }
}

/// A trait to specify the anchor calculation.
pub trait Slugify {
    fn slugify(&mut self, str: String) -> String;
}

/// A slugifier that attempts to mimic GitHub's behavior.
///
/// Unfortunately GitHub's behavior is not documented anywhere by GitHub.
/// This should really be part of the [GitHub Flavored Markdown Spec][gfm]
/// but alas it's not. And there also does not appear to be a public issue
/// tracker for the spec where that issue could be raised.
///
/// [gfm]: https://github.github.com/gfm/
#[derive(Default)]
pub struct GitHubSlugifier {
    counts: HashMap<String, i32>,
}

impl Slugify for GitHubSlugifier {
    fn slugify(&mut self, str: String) -> String {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\w\- ]").unwrap());
        let anchor = RE
            .replace_all(&str.to_lowercase().replace(' ', "-"), "")
            .into_owned();

        let i = self
            .counts
            .entry(anchor.clone())
            .and_modify(|i| *i += 1)
            .or_insert(0);

        match *i {
            0 => anchor,
            i => format!("{}-{}", anchor, i),
        }
        .into()
    }
}

pub fn create_content(site: &Site, publish_drafts: bool) -> Result<()> {
    // Get the list of files
    let files_to_parse: Vec<FilePath> = get_files_for_directory(&site.settings.posts_path);

    // Convert the list of files to Post instances
    // @TODO: Don't load all the md/html in memory but read and write post per post
    let posts: Vec<Post> = get_posts(
        &files_to_parse,
        &site.settings.posts_path,
        &site.settings.blog_prefix_path,
        publish_drafts,
    );

    // Write posts onto disk
    write_posts_html(&posts, site);

    // Sort the posts per indexes fr, en and so on
    let posts_per_indexes: HashMap<String, Vec<&Post>> = get_posts_per_indexes(&posts, site);

    // Write down the list of posts in the index directory fr/, en/, …
    write_indexes_html(posts_per_indexes, site, false);

    let posts_per_tags = get_posts_per_tags(&posts);

    write_indexes_html(posts_per_tags, site, true);

    // Get the list of pages
    let pages_files: Vec<FilePath> = get_files_for_directory(&site.settings.pages_path);

    // Well let's say that a page and a post are the sames for now, we may
    // want to be more generic later on
    let pages: Vec<Post> = get_posts(&pages_files, &site.settings.pages_path, "", publish_drafts);

    write_posts_html(&pages, site);

    let mut posts_and_pages: Vec<Post> = Vec::new();
    posts_and_pages.extend(posts.clone());
    posts_and_pages.extend(pages.clone());

    write_sitemap_for_posts(
        &posts_and_pages,
        &site.settings.base_url.as_str(),
        Path::new(&format!("{}/sitemap.xml", &site.settings.output_path)[..]),
    )?;

    write_atom_for_posts(
        &posts,
        &site.settings.base_url.as_str(),
        &site.settings.author.as_str(),
        &site.settings.website_title.as_str(),
        Path::new(&format!("{}/atom.xml", &site.settings.output_path)[..]),
        &site.settings,
    )?;

    Ok(())
}

pub fn convert_md_to_html(md_content: &str, settings: &Settings, path: Option<&str>) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);

    let mut events = Vec::new();
    let mut code_block: Option<CodeBlock> = None;

    let mut current: Option<CustomHeading> = None;
    let mut slugifier = GitHubSlugifier::default();

    for (event, mut _range) in Parser::new_ext(md_content, options).into_offset_iter() {
        match event {
            Event::Text(text) => {
                if let Some(heading) = current.as_mut() {
                    heading.events.push(Event::Text(text));
                } else {
                    if let Some(ref mut code_block) = code_block {
                        let html = code_block.highlight(&text);
                        events.push(Event::Html(html.into()));
                    } else {
                        events.push(Event::Text(text));
                        continue;
                    }
                }
            }

            Event::Start(Tag::Heading {
                level,
                ref id,
                ref classes,
                ref attrs,
            }) => {
                current = Some(CustomHeading {
                    events: Vec::new(),
                    tag: Tag::Heading {
                        level,
                        id: id.clone(),
                        classes: classes.to_vec(),
                        attrs: attrs.to_vec(),
                    },
                });
            }
            Event::End(TagEnd::Heading(_level)) => {
                let heading = current.take().unwrap();

                if let Tag::Heading {
                    level,
                    ref id,
                    ref classes,
                    ref attrs,
                } = heading.tag
                {
                    let text = heading.text();
                    let string_text = text.to_string();
                    let slug = slugifier.slugify(string_text);
                    events.push(Event::Start(Tag::Heading {
                        level,
                        id: id.clone().or(Some(slug.clone().into())).into(),
                        classes: classes.to_vec(),
                        attrs: attrs.to_vec(),
                    }));
                    let heading_events = heading.events.clone();
                    for e in heading_events {
                        events.push(e);
                    }

                    events.push(Event::End(TagEnd::Heading(level)));
                };
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
            Event::End(TagEnd::CodeBlock) => {
                // reset highlight and close the code block
                code_block = None;
                events.push(Event::Html("</code></pre>\n".into()));
            }
            event => {
                if let Some(heading) = current.as_mut() {
                    heading.events.push(event.clone());
                } else {
                    events.push(event);
                }
            }
        }
    }

    // We remove all the empty things we might have pushed before so we don't get some random \n
    events.retain(|e| match e {
        Event::Text(text) | Event::Html(text) => !text.is_empty(),
        _ => true,
    });

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
            log::error!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                log::error!("Reason: {}", e);
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
pub fn get_posts(
    files_to_parse: &[FilePath],
    posts_path: &str,
    prefix_path: &str,
    publish_drafts: bool,
) -> Vec<Post> {
    files_to_parse
        .iter()
        .filter_map(|file_path| parse_file(file_path, posts_path, prefix_path))
        // Don't publish posts that are still drafts if publish_drafts is false
        .filter(|post| match &post.front_matter.status {
            Some(status) => {
                if publish_drafts {
                    true
                } else {
                    *status != PostStatus::Draft
                }
            }
            None => true,
        })
        .collect()
}

pub fn write_posts_html(posts: &[Post], site: &Site) {
    // For every Post, write the HTML to the correct directory
    for post in posts {
        let html_content = convert_md_to_html(&post.content, &site.settings, Some(&post.path[..]));

        let mut context = Context::new();

        let mut tag_urls: Vec<(String, String)> = get_tag_urls_for_post(
            &post,
            Some(&format!("{}/", &site.settings.blog_prefix_path)),
        )
        .into_iter()
        .collect();

        // Always sort by the key
        tag_urls.sort_by(|a, b| a.0.cmp(&b.0));

        let front_matter = &post.front_matter;
        context.insert("title", &front_matter.title);
        context.insert("created_at", &front_matter.date.to_rfc3339());
        context.insert(
            "updated_at",
            &front_matter.updated_at.map(|date| date.to_rfc3339()),
        );

        context.insert("post_content", &html_content);
        context.insert("url_path", &post.url_path);
        context.insert("tags_urls", &tag_urls);
        context.insert("category", &front_matter.category);

        context.insert("categories", &post.ancestor_directories_names);

        let toc = &post.front_matter.toc.unwrap_or(false);

        if *toc {
            let table_of_contents = TableOfContents::new(&post.content).to_cmark();

            let html_toc = convert_md_to_html(
                table_of_contents.as_str(),
                &site.settings,
                Some(&post.path[..]),
            );
            context.insert("toc", &html_toc);
        }

        context.insert(
            "description",
            &post
                .excerpt
                .clone()
                .map(|e| strip_markdown(e.as_str()))
                .unwrap_or(post.front_matter.title.clone()),
        );

        if let Some(html) = render_template_to_html(context, "blog/post.html", &site.tera) {
            write_post_html(&html, post, &site.settings.output_path);
        };
    }
}

fn get_tag_urls_for_post(post: &Post, prepend: Option<&String>) -> HashMap<String, String> {
    let mut url_per_tags: HashMap<String, String> = HashMap::new();

    let tags = &post.front_matter.tags.clone().unwrap_or_default();

    for tag in tags {
        let key = format!(
            "{}{}/tags/{}/{}",
            prepend.unwrap_or(&"".to_string()),
            post.ancestor_directories_names[0],
            post.ancestor_directories_names[1..].join("/"),
            slugify(tag.to_string())
        );

        url_per_tags.insert(tag.to_string(), key);
    }

    url_per_tags
}

pub fn get_posts_per_tags<'a>(posts: &'a [Post]) -> HashMap<String, Vec<&'a Post>> {
    let mut posts_per_tags: HashMap<String, Vec<&Post>> = HashMap::new();

    // For every Post, let's see if it's part of a prefix we want an index for
    // If it's the case, generate an HashMap of posts for each prefix
    for post in posts {
        let tags = &post.front_matter.tags.clone().unwrap_or_default();

        let urls = get_tag_urls_for_post(post, None);
        for tag in tags {
            let key = urls.get(tag).unwrap();
            let posts_for_tag = posts_per_tags.entry(key.to_string()).or_default();
            posts_for_tag.push(post);
        }
    }

    posts_per_tags
}

pub fn get_posts_per_indexes<'a>(posts: &'a [Post], site: &Site) -> HashMap<String, Vec<&'a Post>> {
    let mut indexes_to_create: HashMap<String, Vec<&Post>> = HashMap::new();

    // For every Post, let's see if it's part of a prefix we want an index for
    // If it's the case, generate an HashMap of posts for each prefix
    for post in posts {
        for directory in &post.ancestor_directories_paths {
            if let Some(dir_index) = directory.strip_prefix(&site.settings.posts_path) {
                if dir_index != "" {
                    if site
                        .settings
                        .create_index_for
                        .contains(&dir_index.to_string())
                    {
                        let key = dir_index.to_string();
                        let posts = indexes_to_create.entry(key).or_default();
                        posts.push(post);
                    }
                }
            };
        }
    }

    indexes_to_create
}

pub fn write_indexes_html(
    indexes_to_create: HashMap<String, Vec<&Post>>,
    site: &Site,
    display_tag_header: bool,
) {
    for (index, mut posts) in indexes_to_create {
        let mut context = Context::new();
        let _ = &posts.sort_by(|p1, p2| p2.front_matter.date.cmp(&p1.front_matter.date));

        let index_parts: Vec<String> = index
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        context.insert("categories", &index_parts);
        context.insert("posts", &posts);
        context.insert("title", &format!("Articles - {}", &index)[..]);

        if display_tag_header {
            let tag_name = if let Some(last_slash_index) = index.rfind('/') {
                // Extract the substring from the position after the last '/' to the end
                &index[last_slash_index + 1..]
            } else {
                &index
            };
            context.insert("tag_name", &tag_name);
        }

        context.insert("url_path", &index);

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
    fs::create_dir_all(output_directory)
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
    log::debug!("## Parsing file: {:?}", file_path);
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let parsed_post = parse_post(content, file_path, input_directory, blog_prefix_path);
            if parsed_post.is_none() {
                log::error!("Unable to parse content of {:?}", file_path);
            }
            parsed_post
        }
        Err(e) => {
            log::error!("Error for {}: {}", &file_path, e);
            None
        }
    }
}

fn get_ancestor_directories_paths(path: &Path) -> Vec<String> {
    path.ancestors()
        .skip(1) // Skip the original path
        .map(|ancestor| ancestor.to_string_lossy().into_owned())
        .filter(|s| !s.is_empty()) // Filter out empty
        // strings (for the root path)
        .collect::<Vec<_>>()
        .into_iter()
        //reverse the order
        .rev()
        .collect()
}

fn get_ancestor_directories_names(path: &Path) -> Vec<String> {
    path.ancestors()
        .skip(1) // Skip the original path
        .filter_map(|ancestor| ancestor.file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
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
    log::debug!("Result data {:?}", result.data);
    let front_matter: Option<FrontMatter> = match result.data {
        Some(data) => match data.deserialize::<FrontMatter>() {
            Ok(front_matter) => {
                let new_front_matter = front_matter;
                Some(new_front_matter)
            }
            Err(e) => {
                log::error!(
                    "Unable to read front matter for file {}. Is it a valid YAML format? Check for example that your title doesn't contain the ':' character.",
                    file_path);
                log::error!("{}", e);
                None
            }
        },
        None => {
            log::error!(
                "No data found in front matter {:?} for file {}",
                content,
                file_path
            );
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

    let mut ancestors_directories_names = get_ancestor_directories_names(Path::new(file_path));
    ancestors_directories_names.retain(|p| p != input_directory);

    front_matter.map(|fm| {
        Post::new(
            fm,
            result.excerpt,
            result.content,
            file_path.to_owned(),
            path_url,
            path_url_encoded,
            get_ancestor_directories_paths(Path::new(file_path)),
            ancestors_directories_names,
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
            vec!["fr/".to_owned(), "fr/content".to_owned()],
            vec!["content".to_owned(), "fr".to_owned()],
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
