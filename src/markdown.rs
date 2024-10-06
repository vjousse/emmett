use crate::codeblock::{CodeBlock, FenceSettings};
use crate::config::Settings;
use crate::slugify::{GitHubSlugifier, Slugify};
use pulldown_cmark::{self as cmark, Event, Options, Parser, Tag, TagEnd};
use std::slice::Iter;

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

pub fn convert_md_to_html(md_content: &str, settings: &Settings, path: Option<&str>) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let mut events = Vec::new();
    let mut code_block: Option<CodeBlock> = None;

    let mut current: Option<CustomHeading> = None;
    let mut slugifier = GitHubSlugifier::default();

    for (event, mut _range) in Parser::new_ext(md_content, options).into_offset_iter() {
        match event {
            Event::Text(text) => {
                if let Some(heading) = current.as_mut() {
                    heading.events.push(Event::Text(text));
                } else if let Some(ref mut code_block) = code_block {
                    let html = code_block.highlight(&text);
                    events.push(Event::Html(html.into()));
                } else {
                    events.push(Event::Text(text));
                    continue;
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

                    events.push(Event::Html("<div class=\"markdown-heading\">\n".into()));
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

                    events.push(Event::Html(format!("<a href=\"#{}\" class=\"anchor\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"m7.775 3.275 1.25-1.25a3.5 3.5 0 1 1 4.95 4.95l-2.5 2.5a3.5 3.5 0 0 1-4.95 0 .751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018 1.998 1.998 0 0 0 2.83 0l2.5-2.5a2.002 2.002 0 0 0-2.83-2.83l-1.25 1.25a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042Zm-4.69 9.64a1.998 1.998 0 0 0 2.83 0l1.25-1.25a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042l-1.25 1.25a3.5 3.5 0 1 1-4.95-4.95l2.5-2.5a3.5 3.5 0 0 1 4.95 0 .751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018 1.998 1.998 0 0 0-2.83 0l-2.5 2.5a1.998 1.998 0 0 0 0 2.83Z\"></path></svg></a>\n", slug).into()));
                    events.push(Event::Html("</div>\n".into()));
                };
            }
            Event::Start(Tag::CodeBlock(ref kind)) => {
                let fence = match kind {
                    cmark::CodeBlockKind::Fenced(fence_info) => FenceSettings::new(fence_info),
                    _ => FenceSettings::new(""),
                };
                let (block, begin) = CodeBlock::new(fence.clone(), settings, path);
                code_block = Some(block);

                events.push(Event::Html("<div class=\"code-block\">".into()));

                let language = &fence.language.unwrap_or("code");

                events.push(Event::Html(
                    format!("<div class=\"language-name\">{}</div>", language).into(),
                ));

                events.push(Event::Html(
                        "<button class=\"copy-to-clipboard\" title=\"Copy to clipboard\"><svg class=\"h-6 w-6 fill-white\" xmlns=\"http://www.w3.org/2000/svg\" x=\"0px\" y=\"0px\" viewBox=\"0 0 24 24\"><path d=\"M 4 2 C 2.895 2 2 2.895 2 4 L 2 17 C 2 17.552 2.448 18 3 18 C 3.552 18 4 17.552 4 17 L 4 4 L 17 4 C 17.552 4 18 3.552 18 3 C 18 2.448 17.552 2 17 2 L 4 2 z M 8 6 C 6.895 6 6 6.895 6 8 L 6 20 C 6 21.105 6.895 22 8 22 L 20 22 C 21.105 22 22 21.105 22 20 L 22 8 C 22 6.895 21.105 6 20 6 L 8 6 z M 8 8 L 20 8 L 20 20 L 8 20 L 8 8 z\"></path></svg></button>\n" .into(),
                ));
                events.push(Event::Html(begin.into()));
            }
            Event::End(TagEnd::CodeBlock) => {
                // reset highlight and close the code block
                code_block = None;
                events.push(Event::Html("</code></pre></div>\n".into()));
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
