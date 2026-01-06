use serde::{Deserialize, Serialize};
use tokio_postgres::{
    types::{FromSqlOwned, ToSql},
    Row,
};

pub mod builder;
pub mod device;
pub mod error;
pub mod pool;

pub use device::{DeviceClient, DeviceRow, UpdateDeviceRow};
pub use error::DbPoolError;
pub use pool::{PoolBuilder, PoolConfig};

pub type QueryResult<T> = Result<T, DbPoolError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(serde::Deserialize, Debug)]
pub struct PaginationQuery {
    pub limit: Option<u32>,
    pub after: Option<String>,
    pub order_by: Option<String>,
    pub direction: Option<Direction>,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Asc => write!(f, "ASC"),
            Direction::Desc => write!(f, "DESC"),
        }
    }
}

impl Direction {
    pub fn reverse(self) -> Self {
        match self {
            Direction::Asc => Direction::Desc,
            Direction::Desc => Direction::Asc,
        }
    }
}

pub struct DbClient {
    pub pool: PoolBuilder,
}

impl DbClient {
    pub async fn new(pool: &PoolBuilder) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn insert_returning(
        &self,
        query: &String,
        params: &[&(dyn ToSql + Sync)],
    ) -> QueryResult<Row> {
        let client = self
            .pool
            .inner
            .get()
            .await
            .map_err(DbPoolError::GetConnectionError)?;
        let stmt = client
            .prepare(query)
            .await
            .map_err(DbPoolError::QueryError)?;
        let row = client
            .query_one(&stmt, params)
            .await
            .map_err(DbPoolError::QueryError)?;
        Ok(row)
    }

    pub async fn paginate<T>(
        &self,
        table: &str,
        cursor_column: &str,
        cursor_value: Option<T>,
        direction: Direction,
        limit: u32,
        where_clause: Option<&str>,
    ) -> QueryResult<(Vec<Row>, Option<T>, bool)>
    where
        T: ToSql + Sync + FromSqlOwned + Copy,
    {
        let mut query = format!("SELECT * FROM {}", table);
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
        let mut param_index = 1;

        // WHERE clause
        if let Some(statement) = where_clause {
            query.push_str(&format!(" WHERE {}", statement));
        } else {
            query.push_str(" WHERE 1=1");
        }

        // Cursor filter
        let cursor_param;
        if let Some(cursor) = cursor_value {
            let op = match direction {
                Direction::Asc => ">",
                Direction::Desc => "<",
            };
            query.push_str(&format!(" AND {} {} ${}", cursor_column, op, param_index));
            cursor_param = Some(cursor);
            param_index += 1;
        } else {
            cursor_param = None;
        }

        if let Some(ref c) = cursor_param {
            params.push(c);
        }

        // ORDER and LIMIT
        let real_limit = std::cmp::min(limit.saturating_add(1), 1000) as i64;
        query.push_str(&format!(
            " ORDER BY {} {} LIMIT ${}",
            cursor_column, direction, param_index
        ));
        params.push(&real_limit);

        // Execute query
        let client = self
            .pool
            .inner
            .get()
            .await
            .map_err(DbPoolError::GetConnectionError)?;
        let stmt = client
            .prepare(&query)
            .await
            .map_err(DbPoolError::QueryError)?;
        let mut rows = client
            .query(&stmt, &params)
            .await
            .map_err(DbPoolError::QueryError)?;

        let has_more = rows.len() as u32 > limit;
        if has_more {
            rows.pop();
        }

        let next_cursor = rows.last().map(|row| row.get::<_, T>(cursor_column));
        Ok((rows, next_cursor, has_more))
    }

    pub async fn ping(&self) -> QueryResult<i32> {
        let client = self
            .pool
            .inner
            .get()
            .await
            .map_err(DbPoolError::GetConnectionError)?;
        let row = client
            .query_one("SELECT 1", &[])
            .await
            .map_err(DbPoolError::QueryError)?;

        Ok(row.get(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dbclient_ping() {
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
            .expect("failed to build pool");
        let db = DbClient::new(&pool).await;

        let result = db.ping().await.expect("ping failed");
        assert_eq!(result, 1);
    }
}
