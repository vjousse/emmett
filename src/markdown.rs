use crate::codeblock::{CodeBlock, FenceSettings};
use crate::config::Settings;
use crate::slugify::{GitHubSlugifier, Slugify};
use pulldown_cmark::{self as cmark, BlockQuoteKind, Event, Options, Parser, Tag, TagEnd};
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
                        id: id.clone().or(Some(slug.clone().into())),
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

            Event::Start(Tag::BlockQuote(kind)) => {
                let (class_str, icon_str) = match kind {
                    None => ("", ""),
                    Some(kind) => match kind {
                        BlockQuoteKind::Note => (
                            " class=\"markdown-alert markdown-alert-note\"",
                            "<p class=\"markdown-alert-title\">\n<svg viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z\"></path></svg>\nNote</p>\n",
                        ),
                        BlockQuoteKind::Tip => {
                            (" class=\"markdown-alert markdown-alert-tip\"", "<p class=\"markdown-alert-title\">\n<svg viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 0 1-1.484.211c-.04-.282-.163-.547-.37-.847a8.456 8.456 0 0 0-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.25 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.751.751 0 0 1-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848.075-.088.147-.173.213-.253.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.75-4-3.75ZM5.75 12h4.5a.75.75 0 0 1 0 1.5h-4.5a.75.75 0 0 1 0-1.5ZM6 15.25a.75.75 0 0 1 .75-.75h2.5a.75.75 0 0 1 0 1.5h-2.5a.75.75 0 0 1-.75-.75Z\"></path></svg>\nTip</p>\n")
                        }
                        BlockQuoteKind::Important => (
                            " class=\"markdown-alert markdown-alert-important\"",
                            "<p class=\"markdown-alert-title\">\n<svg viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0 1 14.25 13H8.06l-2.573 2.573A1.458 1.458 0 0 1 3 14.543V13H1.75A1.75 1.75 0 0 1 0 11.25Zm1.75-.25a.25.25 0 0 0-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h6.5a.25.25 0 0 0 .25-.25v-9.5a.25.25 0 0 0-.25-.25Zm7 2.25v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z\"></path></svg>\nImportant</p>\n",
                        ),
                        BlockQuoteKind::Warning => (
                            " class=\"markdown-alert markdown-alert-warning\"",
                            "<p class=\"markdown-alert-title\">\n<svg class=\"octicon octicon-alert mr-2\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z\"></path></svg>\nWarning</p>\n",
                        ),
                        BlockQuoteKind::Caution => (
                            " class=\"markdown-alert markdown-alert-caution\"",
                            "<p class=\"markdown-alert-title\">\n<svg viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"M4.47.22A.749.749 0 0 1 5 0h6c.199 0 .389.079.53.22l4.25 4.25c.141.14.22.331.22.53v6a.749.749 0 0 1-.22.53l-4.25 4.25A.749.749 0 0 1 11 16H5a.749.749 0 0 1-.53-.22L.22 11.53A.749.749 0 0 1 0 11V5c0-.199.079-.389.22-.53Zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5ZM8 4a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0v-3.5A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z\"></path></svg>\nCaution</p>\n",
                        ),
                    },
                };

                events.push(Event::Html(
                    format!("<blockquote{}>\n{}", class_str, icon_str).into(),
                ));
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
