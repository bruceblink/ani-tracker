mod dao;
mod pool;

pub use pool::get_pg_pool;

use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Postgres, query::QueryAs};

pub async fn run_query<'q, T>(
    pool: &PgPool, // PostgreSQL 连接池
    query: QueryAs<'q, Postgres, T, sqlx::postgres::PgArguments>,
) -> Result<Vec<T>, anyhow::Error>
where
    for<'r> T: FromRow<'r, PgRow> + Send + Unpin,
{
    let rows = query
        .fetch_all(pool)
        .await
        .map_err(|e| anyhow::anyhow!("query error: {:?}", e))?;
    Ok(rows)
}
