use crate::content::create_content;
use crate::errors::Result;
use crate::site::Site;
use crate::templates::filters;

pub fn run() -> Result<()> {
    log::info!("Running the application");
    let site: Result<Site> = Site::new();

    match site {
        Ok(mut site) => {
            site.tera.register_filter(
                "markdown",
                filters::MarkdownFilter::new(site.settings.clone())?,
            );
            create_content(&site);
        }
        Err(e) => {
            log::debug!("{:?}", e);
            log::error!("Unable to create site");
        }
    };

    Ok(())
}
