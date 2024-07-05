use crate::content::create_content;
use crate::errors::Result as ResultCrate;
use crate::site::Site;
use crate::templates::filters;
use anyhow::Result;
use fs_extra::dir::{copy, CopyOptions};

pub fn run(publish_drafts: bool) -> Result<()> {
    log::info!("Running the application");
    let site: ResultCrate<Site> = Site::new();

    match site {
        Ok(mut site) => {
            site.tera.register_filter(
                "markdown",
                filters::MarkdownFilter::new(site.settings.clone())?,
            );
            create_content(&site, publish_drafts)?;
            copy_static(&site.settings.static_path, &site.settings.output_path)?;
        }
        Err(e) => {
            log::debug!("{:?}", e);
            log::error!("Unable to create site");
        }
    };

    Ok(())
}

pub fn copy_static(from: &str, to: &str) -> Result<()> {
    let mut options = CopyOptions::new(); //Initialize default values for CopyOptions
    options.copy_inside = true;
    options.content_only = true;
    options.overwrite = true;
    copy(from, to, &options)?;
    Ok(())
}
