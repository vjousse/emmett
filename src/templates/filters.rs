use crate::config::Settings;
use crate::content::convert_md_to_html;
use std::collections::HashMap;

use tera::{to_value, try_get_value, Filter as TeraFilter, Result as TeraResult, Value};

#[derive(Debug)]
pub struct MarkdownFilter {
    settings: Settings,
}

impl MarkdownFilter {
    pub fn new(settings: Settings) -> TeraResult<Self> {
        Ok(Self { settings })
    }
}

impl TeraFilter for MarkdownFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
        let s = try_get_value!("markdown", "value", String, value);

        let html = convert_md_to_html(&s[..], &self.settings, None);

        Ok(to_value(html).unwrap())
    }
}
