use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(addr);

    let connection_pool = PgPool::connect(
        &configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    run(listener.unwrap(), connection_pool)?.await?;

    Ok(())
}
