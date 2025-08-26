#[cfg(feature = "server")]
mod server_tests {
    use ani_tracker::backend::db::postgresql;
    use ani_tracker::configuration::config::get_configuration;
    use sqlx::{Connection, PgConnection};

    #[tokio::test]
    async fn test_connect_db() -> Result<(), sqlx::Error> {
        let settings = get_configuration().expect("Failed to load configuration");
        let db_settings = settings.database;

        let connection_string = db_settings.connection_string();
        let _conn = PgConnection::connect(&connection_string).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_db_query() -> Result<(), sqlx::Error> {
        let settings = get_configuration().expect("Failed to load configuration");
        let db_settings = settings.database;

        let connection_string = db_settings.connection_string();
        let mut conn = PgConnection::connect(&connection_string).await?;
        let data = sqlx::query!("SELECT * FROM ani_info LIMIT 1")
            .fetch_optional(&mut conn)
            .await?;
        assert!(data.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_db_pool() -> Result<(), sqlx::Error> {
        let connect_pool = postgresql::get_pg_pool();
        let data = sqlx::query!("SELECT * FROM ani_info LIMIT 1")
            .fetch_optional(connect_pool)
            .await?;
        assert!(data.is_none());
        Ok(())
    }
}
