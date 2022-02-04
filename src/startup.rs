use crate::content::list_directory;

pub fn run() {
    log::info!("Running the application");
    list_directory("content");
}
