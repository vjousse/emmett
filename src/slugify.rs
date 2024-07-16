use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

// Mostly based on https://docs.rs/pulldown-cmark-toc/latest/pulldown_cmark_toc/
//
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
