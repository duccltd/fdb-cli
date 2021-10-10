use crate::result::Result;
use crate::error::Error;
use os_type::OSType;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIGURATION_NAME: String = "Config".to_string();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FdbCliConfig {
    // fdb-cli version
    version: u8,

    // path to cluster file
    pub cluster_file: String,
}

pub fn load_config() -> Result<FdbCliConfig> {
    let config = match confy::load::<FdbCliConfig>(&CONFIGURATION_NAME) {
        Ok(res) => {
            println!("Found fdb-cli configuration file (version: {:?})", res.version);
            res
        },
        Err(e) => return Err(
            Error::UnableToReadConfig(e)
        )
    };
    Ok(config)
}

impl std::default::Default for FdbCliConfig {
    fn default() -> Self {
        let path = FdbCliConfig::default_cluster_file();

        Self {
            version: 0,
            cluster_file: String::from(path)
        }
    }
}

impl FdbCliConfig {
    pub fn default_cluster_file() -> &'static str {
        let os_type = os_type::current_platform().os_type;
        match os_type {
            // OS Path
            os_type::OSType::OSX => {
                "/usr/local/etc/foundationdb/fdb.cluster"
            }
            // All other types are unix based systems
            _ => {
                "/etc/foundationdb/fdb.cluster"
            }
        }
    }

    pub fn write(&self) -> Result<()> {
        match confy::store(&CONFIGURATION_NAME, self) {
            Ok(()) => Ok(()),
            Err(e) => return Err(
                Error::UnableToWriteConfig(e)
            )
        }
    }
}

fn to_string(os_type: OSType) -> &'static str {
    match os_type {
        OSType::Ubuntu => "Ubuntu",
        OSType::OSX => "OSX",
        OSType::Alpine => "Alpine",
        OSType::Arch => "Arch",
        OSType::Manjaro => "Manjaro",
        OSType::CentOS => "CentOS",
        OSType::Debian => "Debian",
        OSType::OpenSUSE => "OpenSUSE",
        OSType::Redhat => "Redhat",
        OSType::Unknown => "Unknown",
    }
}