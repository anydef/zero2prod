use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(addr);
    
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    ).await
        .expect("Failed to connect to Postgres");
    run(listener.unwrap(), connection_pool)?.await
}

