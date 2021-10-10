use fdb_cli::{cli, config};
use anyhow::Result;
use fdb_cli::client::FdbClient;
use trompt::Trompt;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = cli::parse();

    #[allow(unused)]
    let guard = unsafe { FdbClient::start_network() }
        .expect("unable to start network");

    let mut config = config::load_config().expect("unable to load config");

    match opts {
        cli::Opts::Move(_params) => {
            let _client = FdbClient::new(&config.cluster_file)
                .expect("unable to start client");

            println!("client started")
        },
        cli::Opts::Delete(_params) => {
            let _client = FdbClient::new(&config.cluster_file)
                .expect("unable to start client");

            println!("client started")
        },
        cli::Opts::Setup(params) => {
            config.cluster_file = params.cluster_file;

            match config.write() {
                Ok(()) => println!("config file has been changed"),
                Err(e) => panic!(e)
            }
        },
        cli::Opts::Reset => {
            let is_sure = Trompt::stdout()
                .confirm("Are you sure [y/n]? ").expect("user declined");

            println!("{}", is_sure);
            if !is_sure {
                std::process::exit(0)
            }
        }
    }

    Ok(())
}
