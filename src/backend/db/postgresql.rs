use once_cell::sync::Lazy;
use sqlx::{PgPool, Pool, Postgres};

pub fn get_pg_pool() -> &'static Pool<Postgres> {
    &PG_POOL
}

static PG_POOL: Lazy<Pool<Postgres>> = Lazy::new(|| {
    let settings =
        crate::configuration::config::get_configuration().expect("Failed to load configuration");
    let db_settings = settings.database;
    let connection_string = db_settings.connection_string();

    // 用 connect_lazy：非阻塞初始化，直到第一次执行 SQL 才真正连接
    PgPool::connect_lazy(&connection_string).expect("Failed to create Postgres connection pool")
});
