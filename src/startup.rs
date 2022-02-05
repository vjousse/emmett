use crate::configuration::get_configuration;
use crate::content::parse_directory;

pub fn run() {
    log::info!("Running the application");
    let configuration = get_configuration().expect("Failed to read configuration.");
    parse_directory(&configuration.input_path, &configuration.output_path);
}
