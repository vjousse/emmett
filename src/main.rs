use emmett::startup::run;
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    match run() {
        Ok(_) => (),
        Err(e) => log::error!("{:?}", e),
    };
}
