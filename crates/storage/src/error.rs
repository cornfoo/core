#[derive(Debug, thiserror::Error)]
pub enum DbPoolError {
    #[error("failed to build the connection pool: {0}")]
    BuildError(#[from] deadpool_postgres::BuildError),

    #[error("database-level error: {0}")]
    DatabaseError(tokio_postgres::Error),

    #[error("failed to get a pooled connection: {0}")]
    GetConnectionError(#[from] deadpool_postgres::PoolError),

    #[error("failed to query the database: {0}")]
    QueryError(#[from] tokio_postgres::Error),

    #[error("configuration error: {0}")]
    ConfigError(String),
}
