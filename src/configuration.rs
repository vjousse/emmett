use config::Config;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub blog_prefix_path: String,
    pub output_path: String,
    pub input_path: String,
    pub create_index_for: Vec<String>,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        // Add configuration values from a file named `configuration`.
        // It will look for any top-level file with an extension
        // that `config` knows how to parse: yaml, json, etc.
        .add_source(config::File::with_name("configuration"))
        // Add in settings from the environment (with a prefix of EM)
        // Eg.. `EM_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("EM"))
        .build()
        .unwrap();

    settings.try_deserialize::<Settings>()
}
