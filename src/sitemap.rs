use crate::post::Post;

use anyhow::Result;
use std::fs::File;
use std::path::Path;

use xml::writer::{EmitterConfig, XmlEvent};

pub fn write_sitemap_for_posts(posts: &[Post], base: &str, path: &Path) -> Result<()> {
    let file = File::create(path).expect("Unable to create file");
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(file);

    let start_event: XmlEvent = XmlEvent::start_element("urlset")
        .attr("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9")
        .into();
    writer.write(start_event)?;

    let mut sorted_posts = posts.to_vec();
    sorted_posts.sort_by(|p1, p2| p1.front_matter.date.cmp(&p2.front_matter.date));

    for post in sorted_posts {
        let start_event: XmlEvent = XmlEvent::start_element("url").into();
        writer.write(start_event)?;

        // <loc>
        let start_event: XmlEvent = XmlEvent::start_element("loc").into();
        writer.write(start_event)?;

        // Avoid //
        let url = format!(
            "{}/{}",
            base,
            if let "/" = post.url_path.as_str() {
                ""
            } else {
                post.url_path.as_str()
            }
        );
        let event = xml::writer::XmlEvent::Characters(url.as_str());
        writer.write(event)?;

        let end_event: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_event)?;

        // </loc>

        // <lastmod>
        let start_event: XmlEvent = XmlEvent::start_element("lastmod").into();
        writer.write(start_event)?;

        let event = xml::writer::XmlEvent::Characters(post.date_rfc3339.as_str());
        writer.write(event)?;

        let end_event: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_event)?;

        // </loc>
        let end_event: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_event)?;
        // </lastmod>
    }

    let end_event: XmlEvent = XmlEvent::end_element().into();
    writer.write(end_event)?;

    Ok(())
}
