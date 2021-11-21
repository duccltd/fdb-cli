use std::collections::HashMap;

use fdb_cli::client::FdbClient;
use fdb_cli::error::Error;
use fdb_cli::result::Result;
use fdb_cli::{cli, config, protobuf::load_protobufs};
use protofish::context::MessageInfo;
use protofish::{decode::PackedArray, prelude::*};
use tracing::*;
use trompt::Trompt;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opts = cli::parse();
    let mut config = config::load_config().expect("unable to load config");

    let proto_context = match &config.proto_file {
        Some(path) => Some(load_protobufs(&path).await?),
        None => None,
    };

    #[allow(unused)]
    let guard = unsafe { FdbClient::start_network() }.expect("unable to start network");

    let client = FdbClient::new(&config.cluster_file).expect("unable to start client");

    match opts {
        /*
        Move a kv pair to another key
         */
        cli::Opts::Move => {}
        /*
        Setup the foundation db config
         */
        cli::Opts::Setup(params) => match params {
            cli::Setup::Set(set) => {
                let mut changed = false;
                if let Some(cluster_file) = set.cluster_file {
                    config.cluster_file = cluster_file;
                    changed = true;
                }

                if let Some(proto_file) = set.proto_file {
                    config.proto_file = Some(proto_file);
                    changed = true;
                }

                if changed {
                    match config.write() {
                        Ok(()) => info!("config file has been changed"),
                        Err(e) => panic!("writing config file: {}", e),
                    }
                } else {
                    println!("Options are cluster-file, proto-file")
                }
            }
            cli::Setup::View => {
                println!("{}", config);
            }
        },
        /*
        Reset foundation db
         */
        cli::Opts::Reset => {
            let is_sure = Trompt::stdout()
                .confirm("Are you sure [y/n]? ")
                .expect("user declined");

            if !is_sure {
                std::process::exit(0)
            }
        }
        /*
        Deleting a key or key range from foundation db
         */
        cli::Opts::Delete(params) => {
            let tx = client.db.create_trx()?;

            match params {
                cli::Space::Key(key) => {
                    let is_sure = Trompt::stdout()
                        .confirm(&format!(
                            "Are you sure you want to delete the key `{}` [y/n]? ",
                            &key.key
                        ))
                        .expect("user declined");

                    if is_sure {
                        client.delete(&tx, key.key.as_bytes());
                        info!("key has been deleted")
                    }
                }
                cli::Space::Range(range) => {
                    let is_sure = Trompt::stdout()
                        .confirm(
                            &format!(
                                "Are you sure you want to delete the range range (start: {}, end: {}) [y/n]? ",
                                range.start,
                                range.end.clone().unwrap_or_else(|| "all".to_string())
                            )
                        ).expect("user declined");

                    if is_sure {
                        client.delete_range(
                            &tx,
                            range.start.as_bytes(),
                            // TODO: Make this optional for deleting all keys
                            range.end.unwrap_or_else(|| "\x00".to_string()).as_bytes(),
                        );
                        info!("key range has been deleted")
                    }
                }
            }
        }
        /*
        Get a protobuf kv pair or range
         */
        cli::Opts::Get(params) => {
            let tx = client.db.create_trx()?;

            match params {
                cli::Space::Key(key) => {
                    let value = match client.get(&tx, key.key.as_bytes()).await {
                        Ok(value) => value,
                        Err(e) => panic!("getting key: {}", e),
                    };

                    let msg = match key.proto {
                        Some(proto) => {
                            let proto_context =
                                proto_context.as_ref().expect("Missing protobuf file");
                            proto_context.get_message(&proto)
                        }
                        None => None,
                    };

                    match value {
                        Some(value) => {
                            println!(
                                "{}",
                                format_kv(&proto_context, msg, key.key.as_bytes().to_vec(), value)?
                            );
                        }
                        None => panic!("Could not find any entries with key: {}", &key.key),
                    }
                }
                cli::Space::Range(range) => {
                    let result = match client
                        .get_range(
                            &tx,
                            range.start.as_bytes(),
                            range.end.unwrap_or_else(|| "\x00".to_string()).as_bytes(),
                        )
                        .await
                    {
                        Ok(result) => result,
                        Err(e) => panic!("getting range: {}", e),
                    };

                    let msg = match range.proto {
                        Some(proto) => {
                            let proto_context =
                                proto_context.as_ref().expect("Missing protobuf file");
                            proto_context.get_message(&proto)
                        }
                        None => None,
                    };

                    for (key, value) in result {
                        println!("{}", format_kv(&proto_context, msg, key, value)?);
                    }
                }
            }
        }
    }

    Ok(())
}

fn format_kv(
    context: &Option<Context>,
    msg: Option<&MessageInfo>,
    key: Vec<u8>,
    value: Vec<u8>,
) -> Result<String> {
    let dec_key = String::from_utf8(key).map_err(Error::StringDecodeError)?;

    match msg {
        None => Ok(String::from_utf8(value).map_err(Error::StringDecodeError)?),
        Some(msg) => {
            let context = context.as_ref().expect("Missing porotobuf context");
            let decoded = msg.decode(&value, context);

            let fields = decoded
                .fields
                .into_iter()
                .map(|field| {
                    let name = &msg.get_field(field.number).unwrap().name;

                    (name.clone(), value_to_string(context, field.value).unwrap())
                })
                .collect();

            let rec = Record {
                key: dec_key,
                fields,
            };

            let enc = serde_json::to_string_pretty(&rec).unwrap();

            Ok(enc)
        }
    }
}

#[derive(serde::Serialize)]
struct Record {
    key: String,
    fields: HashMap<String, serde_json::Value>,
}

#[derive(serde::Serialize)]
struct Field {
    name: String,
    value: serde_json::Value,
}

fn value_to_string(
    context: &protofish::context::Context,
    v: protofish::prelude::Value,
) -> serde_json::Result<serde_json::Value> {
    use protofish::prelude::Value::*;
    use serde_json::to_value;

    Ok(match v {
        Double(v) => to_value(v)?,
        Float(v) => to_value(v)?,
        Int32(v) => to_value(v)?,
        Int64(v) => to_value(v)?,
        UInt32(v) => to_value(v)?,
        UInt64(v) => to_value(v)?,
        SInt32(v) => to_value(v)?,
        SInt64(v) => to_value(v)?,
        Fixed32(v) => to_value(v)?,
        Fixed64(v) => to_value(v)?,
        SFixed32(v) => to_value(v)?,
        SFixed64(v) => to_value(v)?,
        Bool(v) => to_value(v)?,
        String(v) => to_value(v)?,
        Bytes(v) => to_value(v.to_vec())?,

        Packed(v) => match v {
            PackedArray::Double(v) => to_value(v)?,
            PackedArray::Float(v) => to_value(v)?,
            PackedArray::Int32(v) => to_value(v)?,
            PackedArray::Int64(v) => to_value(v)?,
            PackedArray::UInt32(v) => to_value(v)?,
            PackedArray::UInt64(v) => to_value(v)?,
            PackedArray::SInt32(v) => to_value(v)?,
            PackedArray::SInt64(v) => to_value(v)?,
            PackedArray::Fixed32(v) => to_value(v)?,
            PackedArray::Fixed64(v) => to_value(v)?,
            PackedArray::SFixed32(v) => to_value(v)?,
            PackedArray::SFixed64(v) => to_value(v)?,
            PackedArray::Bool(v) => to_value(v)?,
        },

        Message(v) => {
            let resolved = context.resolve_message(v.msg_ref);

            serde_json::Value::Object(
                v.fields
                    .into_iter()
                    .map(|field| {
                        let name = &resolved.get_field(field.number).unwrap().name;

                        (name.clone(), value_to_string(context, field.value).unwrap())
                    })
                    .collect(),
            )
        }

        Enum(v) => {
            let resolved = context.resolve_enum(v.enum_ref);

            let name = resolved.get_field_by_value(v.value).unwrap().name.clone();

            serde_json::Value::String(name)
        }

        // Value which was incomplete due to missing bytes in the payload.
        Incomplete(_, v) => serde_json::Value::String(format!(
            "INCOMPLETE: {}",
            std::string::String::from_utf8_lossy(&v.to_vec()).to_string()
        )),

        // Value which wasn't defined in the context.
        Unknown(v) => serde_json::Value::String(format!("UNKNOWN: {:?}", v)),
    })
}
