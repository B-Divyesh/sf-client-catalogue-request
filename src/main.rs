mod api;

use api::{api_router, AppState};
use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env, net::SocketAddr, path::Path};
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing::info;

const CONTENT_SECURITY_POLICY: &str = "default-src 'self'; img-src 'self' data:; style-src 'self'; script-src 'self'; connect-src 'self' https://api.sociobot.in https://sociobotcustomers.ciamlogin.com; frame-src 'self' https://sociobotcustomers.ciamlogin.com; frame-ancestors 'none'; base-uri 'self'; form-action 'self' https://api.sociobot.in";

struct RuntimeConfig {
    port: u16,
    data_dir: String,
    web_dist: String,
}

fn runtime_config(
    port: Option<String>,
    data_dir: Option<String>,
    web_dist: Option<String>,
) -> RuntimeConfig {
    RuntimeConfig {
        port: port.and_then(|value| value.parse().ok()).unwrap_or(8080),
        data_dir: data_dir.unwrap_or_else(|| "data".into()),
        web_dist: web_dist.unwrap_or_else(|| "dist".into()),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("client_catalogue_request=info,tower_http=info")
        .init();
    let config = runtime_config(
        env::var("PORT").ok(),
        env::var("DATA_DIR").ok(),
        env::var("WEB_DIST").ok(),
    );
    let port = config.port;
    let data_dir = config.data_dir;
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
    let web_dist = config.web_dist;
    let index = format!("{web_dist}/index.html");
    let app_index = index.clone();
    let missing_index = index.clone();
    let app = Router::new()
        .route(
            "/health",
            get({
                let sha = build_sha.clone();
                move || async move { Json(json!({"ok": true, "build_sha": sha})) }
            }),
        )
        .nest("/api", api_router(state))
        .route(
            "/",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .route(
            "/demo",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .route(
            "/demo/inbox",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .route(
            "/privacy",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .route(
            "/terms",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .route(
            "/manage",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .route(
            "/auth/callback",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .route(
            "/c/{token}",
            get({
                let index = app_index.clone();
                move || serve_index(index)
            }),
        )
        .nest_service("/assets", ServeDir::new(format!("{web_dist}/assets")))
        .route_service("/favicon.svg", ServeDir::new(&web_dist))
        .route_service("/robots.txt", ServeDir::new(&web_dist))
        .route_service("/sitemap.xml", ServeDir::new(&web_dist))
        .route_service("/apple-touch-icon.png", ServeDir::new(&web_dist))
        .route_service("/catalogue-template.csv", ServeDir::new(&web_dist))
        .fallback(get(move || not_found(missing_index.clone())))
        .layer(axum::middleware::from_fn(cache_assets))
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static(CONTENT_SECURITY_POLICY),
        ))
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

async fn serve_index(index: String) -> Response {
    match tokio::fs::read(index).await {
        Ok(bytes) => (
            [("content-type", "text/html; charset=utf-8")],
            Body::from(bytes),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
async fn not_found(index: String) -> Response {
    let mut response = serve_index(index).await;
    *response.status_mut() = StatusCode::NOT_FOUND;
    response
}
async fn cache_assets(request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let immutable = request.uri().path().starts_with("/assets/");
    let mut response = next.run(request).await;
    if immutable {
        response.headers_mut().insert(
            "cache-control",
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        );
    }
    response
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_defaults_to_port_and_local_paths() {
        let config = runtime_config(None, None, None);
        assert_eq!(config.port, 8080);
        assert_eq!(config.data_dir, "data");
        assert_eq!(config.web_dist, "dist");
    }

    #[test]
    fn content_security_policy_allows_sociobot_entra() {
        assert!(CONTENT_SECURITY_POLICY
            .split(';')
            .any(|directive| directive.trim() == "connect-src 'self' https://api.sociobot.in https://sociobotcustomers.ciamlogin.com"));
        assert!(CONTENT_SECURITY_POLICY
            .split(';')
            .any(|directive| directive.trim()
                == "frame-src 'self' https://sociobotcustomers.ciamlogin.com"));
    }
}
