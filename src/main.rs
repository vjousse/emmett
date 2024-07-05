use clap::Parser;
use emmett::startup::run;
use env_logger::Env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    publish_drafts: bool,
}
fn main() {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::debug!("Publishing drafts: {:?}", args.publish_drafts);

    match run(args.publish_drafts) {
        Ok(_) => (),
        Err(e) => log::error!("{:?}", e),
    };
}
