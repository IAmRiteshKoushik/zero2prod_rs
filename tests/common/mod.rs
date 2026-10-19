use sqlx::{AssertSqlSafe, Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use zero2prod_rs::configuration::{DatabaseSettings, get_configuration};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration =
        get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string(); // randomize
    let db_pool = configure_database(&configuration.database).await;

    let server = zero2prod_rs::startup::run(listener, db_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection =
        PgConnection::connect(&config.connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres");

    let query = format!(r#"CREATE DATABASE "{}";"#, config.database_name);
    sqlx::query(AssertSqlSafe(query))
        .execute(&mut connection)
        .await
        .expect("Failed to create database");

    // Migrate the database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
