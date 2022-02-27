use crate::config::{get_configuration, Settings};
use crate::errors::Result;
use crate::templates::load_tera;
use tera::Tera;

#[derive(Debug)]
pub struct Site {
    pub settings: Settings,
    pub tera: Tera,
}

impl Site {
    pub fn new() -> Result<Site> {
        let settings: Settings = get_configuration().expect("Failed to read configuration.");
        let tera = load_tera()?;
        let site = Site { settings, tera };

        Ok(site)
    }
}
