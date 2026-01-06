use {
    crate::error::DbPoolError,
    deadpool_postgres::{Config as DeadPoolConfig, Pool as DeadPool, Runtime},
    serde::Deserialize,
    std::{sync::Arc, time::Duration},
    tokio_postgres::NoTls,
};

#[derive(Debug, Clone, Deserialize)]
pub struct PoolConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub max_size: usize,
}

#[derive(Debug, Clone)]
pub struct PoolBuilder {
    pub inner: Arc<DeadPool>,
}

impl PoolBuilder {
    pub async fn new(config: PoolConfig) -> Result<Self, DbPoolError> {
        let mut deadpool_config = DeadPoolConfig::new();
        deadpool_config.host = Some(config.host);
        deadpool_config.port = Some(config.port);
        deadpool_config.dbname = Some(config.name);
        deadpool_config.user = Some(config.user);
        deadpool_config.password = Some(config.password);
        deadpool_config.connect_timeout = Some(Duration::from_secs(10));
        deadpool_config.pool = Some(deadpool_postgres::PoolConfig {
            max_size: config.max_size,
            timeouts: deadpool_postgres::Timeouts {
                wait: Some(Duration::from_secs(2)),
                create: Some(Duration::from_secs(5)),
                recycle: Some(Duration::from_secs(1)),
            },
            ..Default::default()
        });

        let pool = deadpool_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| DbPoolError::ConfigError(e.to_string()))?;

        let client = pool.get().await?;
        client
            .simple_query("SELECT 1")
            .await
            .map_err(DbPoolError::DatabaseError)?;

        Ok(Self {
            inner: Arc::new(pool),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_postgres::SimpleQueryMessage;

    #[tokio::test]
    async fn test_pool_initialization_and_query() {
        let config = PoolConfig {
            host: "localhost".to_string(),
            port: 5432,
            name: "main".to_string(),
            user: "admin".to_string(),
            password: "password".to_string(),
            max_size: 10,
        };

        let pool = PoolBuilder::new(config)
            .await
            .expect("failed to create pool");
        let client = pool.inner.get().await.expect("failed to get client");

        let result = client.simple_query("SELECT 1").await.expect("query failed");

        let rows: Vec<_> = result
            .into_iter()
            .filter_map(|msg| match msg {
                SimpleQueryMessage::Row(row) => Some(row),
                _ => None,
            })
            .collect();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get(0), Some("1"));
    }
}
