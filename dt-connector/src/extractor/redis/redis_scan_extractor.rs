use async_trait::async_trait;
use dt_common::meta::{
    dt_data::DtData,
    position::Position,
    redis::{
        redis_entry::RedisEntry,
        redis_object::{RedisCmd, RedisObject, RedisString},
        redis_statistic_type::RedisStatisticType,
    },
};
use dt_common::{error::Error, log_info, utils::rdb_filter::RdbFilter};

use crate::{
    extractor::{base_extractor::BaseExtractor, redis::redis_resp_types::Value},
    Extractor,
};

use super::redis_client::RedisClient;

pub struct RedisScanExtractor {
    pub base_extractor: BaseExtractor,
    pub statistic_type: RedisStatisticType,
    pub scan_count: u64,
    pub conn: RedisClient,
    pub filter: RdbFilter,
}

#[async_trait]
impl Extractor for RedisScanExtractor {
    async fn extract(&mut self) -> Result<(), Error> {
        log_info!("RedisScanExtractor starts");

        if let RedisStatisticType::HotKey = self.statistic_type {
            let maxmemory_policy = self.get_maxmemory_policy().await?;
            if maxmemory_policy != "allkeys-lfu" {
                return Err(Error::MetadataError(format!(
                    "maxmemory_policy is {}, should be allkeys-lfu",
                    maxmemory_policy
                )));
            }
        }

        let count = &self.scan_count.to_string();
        for db in self.get_dbs().await? {
            if self.filter.filter_db(&db) {
                continue;
            }

            // select db
            let cmd = RedisCmd::from_str_args(&["SELECT", &db]);
            self.conn.send(&cmd).await?;
            if Value::Okay != self.conn.read().await? {
                return Err(Error::RedisResultError(format!("\"SELECT {}\" failed", db)));
            }

            // scan
            let db_id: i64 = db.parse().unwrap();
            let mut cursor = 0;
            loop {
                let cmd = ["SCAN", &cursor.to_string(), "COUNT", count];
                let result = self.query(&cmd).await?;

                cursor = result[0].parse().unwrap();
                for key in result.iter().skip(1) {
                    match self.statistic_type {
                        RedisStatisticType::HotKey => self.analyze_hot_key(db_id, key).await?,
                        RedisStatisticType::BigKey => self.analyze_big_key(db_id, key).await?,
                    }
                }

                if cursor == 0 {
                    break;
                }
            }
        }

        self.base_extractor.wait_task_finish().await
    }

    async fn close(&mut self) -> Result<(), Error> {
        self.conn.close().await
    }
}

impl RedisScanExtractor {
    async fn get_dbs(&mut self) -> Result<Vec<String>, Error> {
        let mut dbs = Vec::new();
        let cmd = ["INFO", "keyspace"];
        let result = self.query(&cmd).await?;
        for line in result[0].lines().skip(1) {
            let tokens: Vec<&str> = line.split(':').collect();
            let db = tokens[0].trim_start_matches("db");
            dbs.push(db.to_string());
        }
        Ok(dbs)
    }

    async fn get_maxmemory_policy(&mut self) -> Result<String, Error> {
        let cmd = ["CONFIG", "GET", "maxmemory-policy"];
        let result = self.query(&cmd).await?;
        if result.len() > 1 {
            return Ok(result[1].clone());
        }
        Ok(String::new())
    }

    async fn analyze_hot_key(&mut self, db_id: i64, key: &str) -> Result<(), Error> {
        let cmd = ["OBJECT", "FREQ", key];
        let result = self.query(&cmd).await?;
        if let Ok(freq) = result[0].parse() {
            let mut entry = RedisEntry::new();
            entry.db_id = db_id;
            entry.key = RedisString::from(key.to_owned());
            entry.freq = freq;
            self.base_extractor
                .push_dt_data(DtData::Redis { entry }, Position::None)
                .await
                .unwrap();
        }
        Ok(())
    }

    async fn analyze_big_key(&mut self, db_id: i64, key: &str) -> Result<(), Error> {
        let cmd = ["MEMORY", "USAGE", key];
        let result = self.query(&cmd).await?;
        let data_size: usize = result[0].parse().unwrap();

        let cmd = ["TYPE", key];
        let result = self.query(&cmd).await?;
        let key_type = result[0].clone();

        let mut entry = RedisEntry::new();
        entry.db_id = db_id;
        entry.key = RedisString::from(key.to_owned());
        entry.data_size = data_size;
        entry.value = RedisObject::new(&key_type);

        self.base_extractor
            .push_dt_data(DtData::Redis { entry }, Position::None)
            .await
    }

    async fn query(&mut self, cmd: &[&str]) -> Result<Vec<String>, Error> {
        self.conn.send(&RedisCmd::from_str_args(cmd)).await?;
        self.conn.read_as_string().await
    }
}
