use crate::config::get_configuration;
use crate::content::create_content;

pub fn run() {
    log::info!("Running the application");
    let configuration = get_configuration().expect("Failed to read configuration.");
    create_content(
        &configuration.input_path,
        &configuration.output_path,
        &configuration.blog_prefix_path,
        &configuration.create_index_for,
        &configuration,
    );
}
