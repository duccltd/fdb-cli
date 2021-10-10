use fdb_cli::{cli, config};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = cli::parse();

    match opts {
        cli::Opts::Move(_params) => {

        },
        cli::Opts::Delete(_params) => {

        },
        cli::Opts::Setup(params) => {
            let mut config = config::load_config().expect("unable to load config");

            config.cluster_file = params.cluster_file;

            match config.write() {
                Ok(()) => println!("config file has been changed"),
                Err(e) => panic!(e)
            }
        }
    }

    Ok(())
}
