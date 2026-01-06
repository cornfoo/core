/**
 * https://github.com/meilisearch/meilisearch/blob/main/crates/meilisearch/src/main.rs
 */
use api::{devices, status};
use settings::Settings;
use storage::{DbClient, PoolBuilder};
use {
    actix_web::{middleware, web, HttpServer},
    std::net::TcpListener,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(web::resource("/devices").route(web::post().to(devices::add_devices)))
            .service(web::resource("/health").route(web::get().to(status::get_health))),
    );
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // TODO: Setup Sentry.

    let settings = Settings::new().expect("Failed to load settings");

    let env_filter = tracing_subscriber::EnvFilter::try_new(&settings.logging.level)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(env_filter)
        .init();

    tracing::info!("Loaded settings: {:?}", settings);

    let pool = PoolBuilder::new(settings.database)
        .await
        .expect("Failed to create database pool");

    let db_client = DbClient::new(&pool).await;
    match db_client.ping().await {
        Ok(result) => {
            tracing::info!("Database connection successful: {}", result);
        }
        Err(e) => {
            tracing::error!("Database connection failed: {}", e);
            std::process::exit(1);
        }
    }

    let addr = format!("{}:{}", settings.api.host, settings.api.port);
    let listener = TcpListener::bind(&addr)?;

    tracing::info!("Starting server on {}", addr);

    let pool_data = web::Data::new(pool);
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(pool_data.clone())
            .configure(configure)
            .wrap(actix_web::middleware::Compress::default())
            .wrap(middleware::Logger::default())
    })
    .listen(listener)?
    .run();

    server.await.expect("server error");

    Ok(())
}
