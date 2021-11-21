use crate::error::Error;
use crate::result::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tracing::*;

const CONFIGURATION_NAME: &str = "fdb-proto-cli";
const VERSION: &str = "0.1.0";

#[derive(Serialize, Deserialize, Debug)]
pub struct FdbCliConfig {
    // fdb-cli version
    version: String,

    // path to cluster file
    pub cluster_file: String,

    // path to the protobuf file
    pub proto_file: Option<String>,
}

pub fn load_config() -> Result<FdbCliConfig> {
    let config = match confy::load::<FdbCliConfig>(CONFIGURATION_NAME) {
        Ok(res) => {
            info!(
                "Found fdb-cli configuration file (version: {:?})",
                res.version
            );
            res
        }
        Err(e) => return Err(Error::UnableToReadConfig(e)),
    };
    Ok(config)
}

impl std::fmt::Display for FdbCliConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Configuration values:
version: {:?}
cluster file: {}
protobuf file: {}",
            self.version,
            &self.cluster_file,
            &self.proto_file.clone().unwrap_or_default()
        )
    }
}

impl std::default::Default for FdbCliConfig {
    fn default() -> Self {
        let path = FdbCliConfig::default_cluster_file();

        Self {
            version: VERSION.to_string(),
            cluster_file: String::from(path),
            proto_file: None,
        }
    }
}

impl FdbCliConfig {
    pub fn default_cluster_file() -> &'static str {
        let os_type = os_type::current_platform().os_type;
        match os_type {
            // OSX Path
            os_type::OSType::OSX => "/usr/local/etc/foundationdb/fdb.cluster",
            // All other types are unix based systems
            _ => "/etc/foundationdb/fdb.cluster",
        }
    }

    pub fn write(&self) -> Result<()> {
        confy::store(CONFIGURATION_NAME, self).map_err(Error::UnableToWriteConfig)
    }
}
