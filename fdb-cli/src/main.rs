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
        cli::Opts::Setup(params) => {
            match params {
                cli::Setup::Set(set) => {
                    config.cluster_file = set.cluster_file;

                    match config.write() {
                        Ok(()) => println!("config file has been changed"),
                        Err(e) => panic!(e)
                    }
                },
                cli::Setup::View => {
                    println!("{}", config);
                }
            }
        },
        cli::Opts::Reset => {
            let is_sure = Trompt::stdout()
                .confirm("Are you sure [y/n]? ").expect("user declined");

            println!("{}", is_sure);
            if !is_sure {
                std::process::exit(0)
            }
        },
        cli::Opts::Delete(params) => {
            let _client = FdbClient::new(&config.cluster_file)
                .expect("unable to start client");

            match params {
                cli::Space::Key(key) => {
                    unimplemented!("key deletion")
                },
                cli::Space::Range(range) => {
                    unimplemented!("range deletion")
                }
            }
        },
        cli::Opts::Get(params) => {
            let client = FdbClient::new(&config.cluster_file)
                .expect("unable to start client");

            let tx = client.db.create_trx()?;

            match params {
                cli::Space::Key(key) => {
                    let value = match client.get(&tx, &key.key.as_bytes()).await {
                        Ok(value) => value,
                        Err(e) => panic!(e)
                    };
                    match value {
                        Some(value) => {
                            let result = std::str::from_utf8(&value)
                                .expect("unable to decode protobuf");
                            println!("{}", result)
                        },
                        None => panic!("Could not find any entries with key: {}", &key.key)
                    }

                }
                cli::Space::Range(range) => {
                    let result = match client.get_range(
                        &tx,
                        &range.start.as_bytes(),
                        &range.end.unwrap_or("\x00".to_string()).as_bytes()
                    ).await {
                        Ok(result) => result,
                        Err(e) => panic!(e)
                    };

                    for (key, value) in result {
                        let dec_key = std::str::from_utf8(&key)
                            .expect("unable to decode protobuf");
                        let dec_value = std::str::from_utf8(&value)
                            .expect("unable to decode protobuf");
                        println!("{}: {}", dec_key, dec_value);
                    }
                }
            }
        }
    }

    Ok(())
}
