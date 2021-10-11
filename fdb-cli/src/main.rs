use fdb_cli::{cli, config};
use anyhow::Result;
use fdb_cli::client::FdbClient;
use trompt::Trompt;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = cli::parse();

    #[allow(unused)]
    let guard = unsafe { FdbClient::start_network() }
        .expect("unable to start network");

    let mut config = config::load_config().expect("unable to load config");

    let client = FdbClient::new(&config.cluster_file)
        .expect("unable to start client");

    match opts {
        /*
        Move a kv pair to another key
         */
        cli::Opts::Move(_params) => {},
        /*
        Setup the foundation db config
         */
        cli::Opts::Setup(params) => {
            match params {
                cli::Setup::Set(set) => {
                    config.cluster_file = set.cluster_file;

                    match config.write() {
                        Ok(()) => info!("config file has been changed"),
                        Err(e) => panic!(e)
                    }
                },
                cli::Setup::View => {
                    info!("{}", config);
                }
            }
        },
        /*
        Reset foundation db
         */
        cli::Opts::Reset => {
            let is_sure = Trompt::stdout()
                .confirm("Are you sure [y/n]? ").expect("user declined");

            if !is_sure {
                std::process::exit(0)
            }
        },
        /*
        Deleting a key or key range from foundation db
         */
        cli::Opts::Delete(params) => {
            let tx = client.db.create_trx()?;

            match params {
                cli::Space::Key(key) => {
                    let is_sure = Trompt::stdout()
                        .confirm(
                            &format!(
                                "Are you sure you want to delete the key `{}` [y/n]? ",
                                &key.key)
                        ).expect("user declined");

                    if is_sure {
                        client.delete(&tx, &key.key.as_bytes());
                        info!("key has been deleted")
                    }
                },
                cli::Space::Range(range) => {
                    let is_sure = Trompt::stdout()
                        .confirm(
                            &format!(
                                "Are you sure you want to delete the range range (start: {}, end: {}) [y/n]? ",
                                range.start.clone(),
                                range.end.clone().unwrap_or("all".to_string())
                            )
                        ).expect("user declined");

                    if is_sure {
                        client.delete_range(
                            &tx,
                            &range.start.as_bytes(),

                            // TODO: Make this optional for deleting all keys
                            &range.end.unwrap_or("\x00".to_string()).as_bytes()
                        );
                        info!("key range has been deleted")
                    }
                }
            }
        },
        /*
        Get a protobuf kv pair or range
         */
        cli::Opts::Get(params) => {
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
                            info!("{}", result)
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
                        info!("{}: {}", dec_key, dec_value);
                    }
                }
            }
        }
    }

    Ok(())
}
