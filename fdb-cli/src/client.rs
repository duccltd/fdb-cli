use foundationdb::{Database, Transaction, RangeOption};
use foundationdb::api::{NetworkAutoStop, FdbApiBuilder};
use crate::result::Result;
use tokio::time::timeout;
use std::time::Duration;
use foundationdb::future::FdbValues;

pub struct FdbClient {
    pub db: Database,
}

impl FdbClient {
    pub unsafe fn start_network() -> Result<NetworkAutoStop> {
        let network_builder = FdbApiBuilder::default().build()?;

        network_builder.boot().map_err(Into::into)
    }

    pub fn new(path: &str) -> Result<Self> {
        let db = Database::new(Some(path))?;
        Ok(Self { db })
    }

    pub async fn begin_tx(&self) -> Result<Transaction> {
        let f = async { self.db.create_trx().map_err(Into::into) };

        match timeout(Duration::from_secs(1), f).await {
            Ok(tx) => tx,
            Err(e) => Err(e.into())
        }
    }

    pub fn delete<'a>(&self, tx: &'a Transaction, key: &'a [u8]) {
        tx.clear(key);
    }

    pub fn set<'a>(&self, tx: &'a Transaction, key: &'a [u8], value: &[u8]) {
        println!("set {}", String::from_utf8_lossy(key));
        tx.set(key, value);
    }

    pub async fn get<'a>(&self, tx: &'a Transaction, key: &'a [u8]) -> Result<Option<Vec<u8>>> {
        println!("get {}", String::from_utf8_lossy(key));
        let opt_val = tx.get(key, false).await?;
        let val = match opt_val {
            None => return Ok(None),
            Some(val) => val,
        };
        Ok(Some((*val).to_vec()))
    }

    pub async fn get_range_keys<'a>(
        &self,
        tx: &'a Transaction,
        from: &'a [u8],
        to: &'a [u8]
    ) -> Result<FdbValues> {
        println!(
            "get_range_keys {} {}",
            String::from_utf8_lossy(from),
            String::from_utf8_lossy(to),
        );

        let values = tx
            .get_range(
                &RangeOption {
                    reverse: false,
                    limit: None,
                    mode: foundationdb::options::StreamingMode::WantAll,
                    ..RangeOption::from((from, to))
                },
                1,
                false,
            )
            .await?;

        // Ok(values.into_iter().map(|kv| kv.key().to_vec()).collect())
        Ok(values)
    }
}