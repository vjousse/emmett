pub mod filters;

use crate::errors::Result;
use tera::Tera;

pub fn load_tera() -> Result<Tera> {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![]);
    //tera.register_filter("markdown", markdown_filter);
    Ok(tera)
}
