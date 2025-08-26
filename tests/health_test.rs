#[cfg(feature = "server")]
mod server_tests {
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
}