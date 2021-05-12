use r2d2_redis::redis::{Commands, FromRedisValue, Value};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::time::Duration;
use thiserror::Error;
use crate::model::Experiment;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;
pub type RedisConnection = r2d2::PooledConnection<RedisConnectionManager>;
type RedisResult<T> = std::result::Result<T, RedisError>;

const CACHE_POOL_MAX_SIZE: u32 = 16;
const CACHE_POOL_MIN_IDLE: u32 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_MAX_LIFETIME: u64 = 60;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("Issue with Redis Pool : {0}")]
    PoolError(r2d2_redis::r2d2::Error),
    #[error("Parsing Error: {0}")]
    TypeError(r2d2_redis::redis::RedisError),
    #[error("Execution Error: {0}")]
    CMDError(r2d2_redis::redis::RedisError),
    #[error("Redis Client Error: {0}")]
    RedisClientError(r2d2_redis::redis::RedisError),
}

pub fn connect(connection_string: &str) -> RedisResult<RedisPool> {
    let manager = RedisConnectionManager::new(connection_string).map_err(RedisError::RedisClientError)?;
    r2d2::Pool::builder()
        .max_size(CACHE_POOL_MAX_SIZE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_MAX_LIFETIME)))
        .min_idle(Some(CACHE_POOL_MIN_IDLE))
        .build(manager)
        .map_err(|e| RedisError::PoolError(e).into())
}
pub fn get_con(pool: &RedisPool) -> RedisResult<RedisConnection> {
    pool.get_timeout(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS))
        .map_err(|e| {
            eprintln!("error connecting to redis: {}", e);
            RedisError::PoolError(e).into()
        })
}
fn save_str(pool: &RedisPool, key: &str, value: &str) -> RedisResult<()> {
    let mut con = get_con(&pool)?;
    con.set(key, value).map_err(RedisError::CMDError)?;
    Ok(())
}

pub fn save_experiment(pool: &RedisPool, key: &str, experiment: &Experiment) -> RedisResult<()> {
    let serialized = serde_json::to_string(&experiment).unwrap();
    save_str(&pool, key, &serialized)?;
    Ok(())
} 

fn deserialize_experiment(value: String) -> Experiment {
    serde_json::from_str(&value).unwrap()
}

pub fn get_experiment(pool: &RedisPool, key: &str) -> RedisResult<Experiment> {
    let mut con = get_con(&pool)?;
    let value  = con.get(key).map_err(RedisError::CMDError)?;
    match value {
        Value::Nil => Ok(Experiment::empty()),
        _ => FromRedisValue::from_redis_value(&value)
            .map(|v: String| deserialize_experiment(v))
            .map_err(|e| RedisError::TypeError(e).into())            
    }
}
