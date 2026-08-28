mod api;

use api::{api_router, AppState};
use axum::{
    http::{HeaderName, HeaderValue},
    routing::get,
    Json, Router,
};
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, net::SocketAddr, path::Path};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("client_catalogue_request=info,tower_http=info")
        .init();
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".into());
    tokio::fs::create_dir_all(&data_dir).await?;
    let db_path = format!("{data_dir}/catalogue.db");
    if !Path::new(&db_path).exists() {
        tokio::fs::File::create(&db_path).await?;
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect(&format!("sqlite://{db_path}"))
        .await?;
    sqlx::migrate!().run(&pool).await?;
    let state = AppState::new(pool);
    let build_sha = option_env!("BUILD_SHA").unwrap_or("dev").to_string();
    let web_dist = env::var("WEB_DIST").unwrap_or_else(|_| "dist".into());
    let index = format!("{web_dist}/index.html");
    let app = Router::new()
        .route("/health", get({ let sha = build_sha.clone(); move || async move { Json(json!({"ok": true, "build_sha": sha})) } }))
        .nest("/api", api_router(state))
        .fallback_service(ServeDir::new(&web_dist).fallback(ServeFile::new(index)))
        .layer(SetResponseHeaderLayer::if_not_present(HeaderName::from_static("x-content-type-options"), HeaderValue::from_static("nosniff")))
        .layer(SetResponseHeaderLayer::if_not_present(HeaderName::from_static("referrer-policy"), HeaderValue::from_static("strict-origin-when-cross-origin")))
        .layer(SetResponseHeaderLayer::if_not_present(HeaderName::from_static("content-security-policy"), HeaderValue::from_static("default-src 'self'; img-src 'self' data:; style-src 'self'; script-src 'self'; connect-src 'self' https://api.sociobot.in; frame-ancestors 'none'; base-uri 'self'; form-action 'self' https://api.sociobot.in")))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(%addr, %db_path, "configuration loaded; no secret environment variables required");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown())
    .await?;
    Ok(())
}

async fn shutdown() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("signal handler") };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}
