use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::{ConnectInfo, DefaultBodyLimit, Path, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use dashmap::DashMap;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Row, SqlitePool};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    rates: Arc<DashMap<String, (Instant, u32)>>,
    demos: Arc<DashMap<String, DemoWorkspace>>,
}
#[derive(Clone)]
struct DemoWorkspace {
    created_at: Instant,
    requests: Vec<Value>,
}
impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            rates: Arc::new(DashMap::new()),
            demos: Arc::new(DashMap::new()),
        }
    }
}
type ApiResult<T> = Result<T, (StatusCode, Json<Value>)>;
fn err(code: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (code, Json(json!({"error": message})))
}
fn token(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/setup/status", get(setup_status))
        .route("/setup", post(setup))
        .route("/login", post(login))
        .route("/admin/catalogue", get(admin_catalogue).put(save_catalogue))
        .route("/admin/links", post(create_link))
        .route("/admin/requests", get(list_requests))
        .route("/catalogue/{token}", get(client_catalogue))
        .route("/requests/{token}", post(create_request))
        .route("/demo", post(create_demo))
        .route(
            "/demo/{id}/requests",
            get(demo_requests)
                .post(create_demo_request)
                .delete(delete_demo),
        )
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit))
        .with_state(state)
}

const DEMO_TTL: Duration = Duration::from_secs(24 * 60 * 60);

fn sample_demo_request() -> Value {
    json!({
        "id":"RQ-6C24A19E", "client_name":"Maya Patel", "company":"Juniper Corner",
        "email":"maya@example.test", "po_number":"PO-1842",
        "note":"Please quote delivery to Bristol.", "status":"New", "created_at":"2026-08-28 09:14",
        "lines":[
            {"product_id":"p1", "sku":"NW-101", "name":"Recycled counter notebook", "quantity":24, "price_cents":850},
            {"product_id":"p4", "sku":"PK-228", "name":"Custom paper tape", "quantity":12, "price_cents":null}
        ]
    })
}

async fn create_demo(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    state
        .demos
        .retain(|_, demo| demo.created_at.elapsed() < DEMO_TTL);
    let id = token(32);
    state.demos.insert(
        id.clone(),
        DemoWorkspace {
            created_at: Instant::now(),
            requests: vec![sample_demo_request()],
        },
    );
    (
        StatusCode::CREATED,
        Json(json!({"id":id, "expires_in_seconds":DEMO_TTL.as_secs()})),
    )
}

fn demo_not_found() -> (StatusCode, Json<Value>) {
    err(
        StatusCode::NOT_FOUND,
        "This sample workspace expired. Reset the demo to start again.",
    )
}

async fn demo_requests(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Value>> {
    let demo = state.demos.get(&id).ok_or_else(demo_not_found)?;
    if demo.created_at.elapsed() >= DEMO_TTL {
        drop(demo);
        state.demos.remove(&id);
        return Err(demo_not_found());
    }
    Ok(Json(json!({"requests":demo.requests.clone()})))
}

async fn delete_demo(State(state): State<AppState>, Path(id): Path<String>) -> StatusCode {
    state.demos.remove(&id);
    StatusCode::NO_CONTENT
}

async fn rate_limit(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_owned())
        .unwrap_or_else(|| peer.ip().to_string());
    let key = ip;
    let now = Instant::now();
    let mut entry = state.rates.entry(key).or_insert((now, 0));
    if now.duration_since(entry.0) >= Duration::from_secs(1) {
        *entry = (now, 0);
    }
    entry.1 += 1;
    let limit =
        if req.method() == axum::http::Method::POST || req.method() == axum::http::Method::PUT {
            12
        } else {
            40
        };
    if entry.1 > limit {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, HeaderValue::from_static("1"))],
            Json(json!({"error":"Too many requests. Wait one second and try again."})),
        )
            .into_response();
    }
    drop(entry);
    next.run(req).await
}

#[derive(Deserialize)]
struct SetupInput {
    business_name: String,
    password: String,
}
async fn setup_status(State(state): State<AppState>) -> ApiResult<Json<Value>> {
    let claimed: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM owner")
        .fetch_one(&state.pool)
        .await
        .map_err(internal)?;
    Ok(Json(json!({"claimed": claimed > 0})))
}
async fn setup(
    State(state): State<AppState>,
    Json(input): Json<SetupInput>,
) -> ApiResult<Json<Value>> {
    if input.business_name.trim().len() < 2 || input.business_name.len() > 80 {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Enter a business name between 2 and 80 characters.",
        ));
    }
    if input.password.len() < 10 || input.password.len() > 200 {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Use a password from 10 to 200 characters.",
        ));
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM owner")
        .fetch_one(&state.pool)
        .await
        .map_err(internal)?;
    if count > 0 {
        return Err(err(
            StatusCode::CONFLICT,
            "This workspace already has an owner. Sign in instead.",
        ));
    }
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|_| {
            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not secure the password. Try again.",
            )
        })?
        .to_string();
    let mut tx = state.pool.begin().await.map_err(internal)?;
    sqlx::query("INSERT INTO owner(id,password_hash) VALUES(1,?)")
        .bind(hash)
        .execute(&mut *tx)
        .await
        .map_err(internal)?;
    sqlx::query("UPDATE settings SET business_name=? WHERE id=1")
        .bind(input.business_name.trim())
        .execute(&mut *tx)
        .await
        .map_err(internal)?;
    tx.commit().await.map_err(internal)?;
    Ok(Json(json!({"token": new_session(&state.pool).await?})))
}
#[derive(Deserialize)]
struct LoginInput {
    password: String,
}
async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginInput>,
) -> ApiResult<Json<Value>> {
    if input.password.len() > 200 {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "The password is too long. Use no more than 200 characters.",
        ));
    }
    let row = sqlx::query("SELECT password_hash FROM owner WHERE id=1")
        .fetch_optional(&state.pool)
        .await
        .map_err(internal)?
        .ok_or_else(|| {
            err(
                StatusCode::NOT_FOUND,
                "Set up the workspace before signing in.",
            )
        })?;
    let hash: String = row.get(0);
    if Argon2::default()
        .verify_password(
            input.password.as_bytes(),
            &PasswordHash::new(&hash).map_err(|_| {
                err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "The saved password could not be read.",
                )
            })?,
        )
        .is_err()
    {
        return Err(err(
            StatusCode::UNAUTHORIZED,
            "That password does not match. Check it and try again.",
        ));
    }
    Ok(Json(json!({"token": new_session(&state.pool).await?})))
}
async fn new_session(pool: &SqlitePool) -> ApiResult<String> {
    let value = token(48);
    let expiry = chrono::Utc::now() + chrono::Duration::days(30);
    sqlx::query("INSERT INTO sessions(token,expires_at) VALUES(?,?)")
        .bind(&value)
        .bind(expiry.to_rfc3339())
        .execute(pool)
        .await
        .map_err(internal)?;
    Ok(value)
}
async fn require_admin(headers: &HeaderMap, pool: &SqlitePool) -> ApiResult<()> {
    let value = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| {
            err(
                StatusCode::UNAUTHORIZED,
                "Sign in to open the seller workspace.",
            )
        })?;
    let found: Option<String> =
        sqlx::query_scalar("SELECT token FROM sessions WHERE token=? AND expires_at > ?")
            .bind(value)
            .bind(chrono::Utc::now().to_rfc3339())
            .fetch_optional(pool)
            .await
            .map_err(internal)?;
    if found.is_none() {
        return Err(err(
            StatusCode::UNAUTHORIZED,
            "Your sign-in expired. Sign in again.",
        ));
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    id: String,
    sku: String,
    name: String,
    description: String,
    category: String,
    price_cents: Option<i64>,
    stock_note: String,
}
#[derive(Serialize, Deserialize)]
struct Settings {
    business_name: String,
    price_label: String,
    tax_note: String,
    currency: String,
}
#[derive(Serialize, Deserialize)]
struct CataloguePayload {
    settings: Settings,
    products: Vec<Product>,
    links: Option<Vec<ClientLink>>,
}
#[derive(Serialize, Deserialize)]
struct ClientLink {
    token: String,
    label: String,
    active: bool,
}

async fn admin_catalogue(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> ApiResult<Json<CataloguePayload>> {
    require_admin(&headers, &state.pool).await?;
    Ok(Json(load_catalogue(&state.pool, None, true).await?))
}
#[derive(Deserialize)]
struct SavePayload {
    settings: Settings,
    products: Vec<Product>,
}
async fn save_catalogue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SavePayload>,
) -> ApiResult<Json<Value>> {
    require_admin(&headers, &state.pool).await?;
    if payload.products.len() > 5000 {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "This import has more than 5,000 rows. Split it into smaller files.",
        ));
    }
    validate_settings(&payload.settings)?;
    let mut tx = state.pool.begin().await.map_err(internal)?;
    sqlx::query(
        "UPDATE settings SET business_name=?,price_label=?,tax_note=?,currency=? WHERE id=1",
    )
    .bind(payload.settings.business_name.trim())
    .bind(payload.settings.price_label.trim())
    .bind(payload.settings.tax_note.trim())
    .bind(payload.settings.currency.trim().to_uppercase())
    .execute(&mut *tx)
    .await
    .map_err(internal)?;
    sqlx::query("DELETE FROM products")
        .execute(&mut *tx)
        .await
        .map_err(internal)?;
    for (index, p) in payload.products.iter().enumerate() {
        if p.sku.trim().is_empty()
            || p.name.trim().is_empty()
            || p.sku.len() > 80
            || p.name.len() > 120
            || p.description.len() > 1000
            || p.category.len() > 80
            || p.stock_note.len() > 200
            || p.price_cents.is_some_and(|v| v < 0)
        {
            return Err(err(
                StatusCode::BAD_REQUEST,
                &format!(
                    "Row {} needs a SKU, a short name, and a valid price or POA.",
                    index + 1
                ),
            ));
        }
        sqlx::query("INSERT INTO products(id,sku,name,description,category,price_cents,stock_note) VALUES(?,?,?,?,?,?,?)")
            .bind(if p.id.is_empty() { uuid::Uuid::new_v4().to_string() } else { p.id.clone() }).bind(p.sku.trim()).bind(p.name.trim()).bind(p.description.trim()).bind(p.category.trim()).bind(p.price_cents).bind(p.stock_note.trim())
            .execute(&mut *tx).await.map_err(|_| err(StatusCode::BAD_REQUEST, &format!("SKU {} appears more than once.", p.sku)))?;
    }
    tx.commit().await.map_err(internal)?;
    Ok(Json(
        json!({"saved": true, "count": payload.products.len()}),
    ))
}
fn validate_settings(s: &Settings) -> ApiResult<()> {
    if s.business_name.trim().is_empty()
        || s.business_name.len() > 80
        || s.price_label.trim().is_empty()
        || s.price_label.len() > 40
        || s.tax_note.len() > 160
        || s.currency.trim().len() != 3
    {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Add a business name, price label, and three-letter currency code.",
        ));
    }
    Ok(())
}
#[derive(Deserialize)]
struct LinkInput {
    label: String,
}
async fn create_link(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<LinkInput>,
) -> ApiResult<Json<ClientLink>> {
    require_admin(&headers, &state.pool).await?;
    if input.label.trim().len() < 2 || input.label.len() > 120 {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Name the client or client group for this link.",
        ));
    }
    let link = ClientLink {
        token: token(28),
        label: input.label.trim().to_string(),
        active: true,
    };
    sqlx::query("INSERT INTO client_links(token,label) VALUES(?,?)")
        .bind(&link.token)
        .bind(&link.label)
        .execute(&state.pool)
        .await
        .map_err(internal)?;
    Ok(Json(link))
}
async fn client_catalogue(
    State(state): State<AppState>,
    Path(link_token): Path<String>,
) -> ApiResult<Json<CataloguePayload>> {
    Ok(Json(
        load_catalogue(&state.pool, Some(&link_token), false).await?,
    ))
}
async fn load_catalogue(
    pool: &SqlitePool,
    link_token: Option<&str>,
    include_links: bool,
) -> ApiResult<CataloguePayload> {
    if let Some(t) = link_token {
        let active: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM client_links WHERE token=? AND active=1")
                .bind(t)
                .fetch_one(pool)
                .await
                .map_err(internal)?;
        if active == 0 {
            return Err(err(
                StatusCode::NOT_FOUND,
                "This client link is not active. Ask the seller for a new link.",
            ));
        }
    }
    let s =
        sqlx::query("SELECT business_name,price_label,tax_note,currency FROM settings WHERE id=1")
            .fetch_one(pool)
            .await
            .map_err(internal)?;
    let rows = sqlx::query("SELECT id,sku,name,description,category,price_cents,stock_note FROM products WHERE active=1 ORDER BY category,name").fetch_all(pool).await.map_err(internal)?;
    let products = rows
        .into_iter()
        .map(|r| Product {
            id: r.get(0),
            sku: r.get(1),
            name: r.get(2),
            description: r.get(3),
            category: r.get(4),
            price_cents: r.get(5),
            stock_note: r.get(6),
        })
        .collect();
    let links = if include_links {
        Some(
            sqlx::query("SELECT token,label,active FROM client_links ORDER BY created_at DESC")
                .fetch_all(pool)
                .await
                .map_err(internal)?
                .into_iter()
                .map(|r| ClientLink {
                    token: r.get(0),
                    label: r.get(1),
                    active: r.get::<i64, _>(2) == 1,
                })
                .collect(),
        )
    } else {
        None
    };
    Ok(CataloguePayload {
        settings: Settings {
            business_name: s.get(0),
            price_label: s.get(1),
            tax_note: s.get(2),
            currency: s.get(3),
        },
        products,
        links,
    })
}

#[derive(Deserialize)]
struct RequestInput {
    client_name: String,
    company: String,
    email: String,
    po_number: String,
    note: String,
    lines: Vec<RequestLineInput>,
}
#[derive(Deserialize)]
struct RequestLineInput {
    product_id: String,
    quantity: i64,
}
fn validate_request(input: &RequestInput) -> ApiResult<()> {
    if input.client_name.trim().len() < 2
        || input.client_name.len() > 120
        || input.company.trim().len() < 2
        || input.company.len() > 120
        || !input.email.contains('@')
        || input.email.len() > 254
        || input.po_number.len() > 80
        || input.note.len() > 2000
    {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Add your name, company, and a valid email address.",
        ));
    }
    if input.lines.is_empty()
        || input.lines.len() > 100
        || input
            .lines
            .iter()
            .any(|line| line.quantity < 1 || line.quantity > 9999)
    {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Add at least one product. Each quantity must be from 1 to 9,999.",
        ));
    }
    let mut product_ids = HashSet::new();
    if input
        .lines
        .iter()
        .any(|line| !product_ids.insert(&line.product_id))
    {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Each product can appear only once. Change its quantity instead.",
        ));
    }
    Ok(())
}

async fn create_demo_request(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<RequestInput>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    validate_request(&input)?;
    let products = [
        ("p1", "NW-101", "Recycled counter notebook", Some(850)),
        ("p2", "NW-114", "Brass desk ruler", Some(1250)),
        ("p3", "PK-220", "Kraft dispatch box", Some(1890)),
        ("p4", "PK-228", "Custom paper tape", None),
        ("p5", "SV-410", "Shelf label set", Some(2400)),
        ("p6", "SV-421", "Oak display riser", None),
    ];
    let mut lines = Vec::new();
    for line in &input.lines {
        let product = products
            .iter()
            .find(|product| product.0 == line.product_id)
            .ok_or_else(|| {
                err(
                    StatusCode::BAD_REQUEST,
                    "A selected sample product is not available. Reset the demo.",
                )
            })?;
        lines.push(json!({
            "product_id":product.0, "sku":product.1, "name":product.2,
            "quantity":line.quantity, "price_cents":product.3
        }));
    }
    let request_id = format!(
        "RQ-DEMO-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..4].to_uppercase()
    );
    let request = json!({
        "id":request_id, "client_name":input.client_name.trim(), "company":input.company.trim(),
        "email":input.email.trim(), "po_number":input.po_number.trim(), "note":input.note.trim(),
        "status":"New", "created_at":chrono::Utc::now().to_rfc3339(), "lines":lines
    });
    let mut demo = state.demos.get_mut(&id).ok_or_else(demo_not_found)?;
    if demo.created_at.elapsed() >= DEMO_TTL {
        drop(demo);
        state.demos.remove(&id);
        return Err(demo_not_found());
    }
    demo.requests.insert(0, request);
    Ok((StatusCode::CREATED, Json(json!({"id":request_id}))))
}

async fn create_request(
    State(state): State<AppState>,
    Path(link_token): Path<String>,
    Json(input): Json<RequestInput>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let active: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM client_links WHERE token=? AND active=1")
            .bind(&link_token)
            .fetch_one(&state.pool)
            .await
            .map_err(internal)?;
    if active == 0 {
        return Err(err(
            StatusCode::NOT_FOUND,
            "This client link is not active. Ask the seller for a new link.",
        ));
    }
    validate_request(&input)?;
    let id = format!(
        "RQ-{}",
        uuid::Uuid::new_v4().simple().to_string()[..8].to_uppercase()
    );
    let mut tx = state.pool.begin().await.map_err(internal)?;
    sqlx::query("INSERT INTO quote_requests(id,link_token,client_name,company,email,po_number,note) VALUES(?,?,?,?,?,?,?)")
        .bind(&id).bind(&link_token).bind(input.client_name.trim()).bind(input.company.trim()).bind(input.email.trim()).bind(input.po_number.trim()).bind(input.note.trim()).execute(&mut *tx).await.map_err(internal)?;
    for line in input.lines {
        let p = sqlx::query("SELECT sku,name,price_cents FROM products WHERE id=? AND active=1")
            .bind(&line.product_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(internal)?
            .ok_or_else(|| {
                err(
                    StatusCode::BAD_REQUEST,
                    "A selected product is no longer available. Reload the catalogue.",
                )
            })?;
        sqlx::query("INSERT INTO request_lines(request_id,product_id,sku,name,quantity,price_cents) VALUES(?,?,?,?,?,?)").bind(&id).bind(&line.product_id).bind(p.get::<String,_>(0)).bind(p.get::<String,_>(1)).bind(line.quantity).bind(p.get::<Option<i64>,_>(2)).execute(&mut *tx).await.map_err(internal)?;
    }
    tx.commit().await.map_err(internal)?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"id": id, "message":"Request received"})),
    ))
}
async fn list_requests(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> ApiResult<Json<Value>> {
    require_admin(&headers, &state.pool).await?;
    let reqs = sqlx::query("SELECT id,client_name,company,email,po_number,note,status,created_at FROM quote_requests ORDER BY created_at DESC").fetch_all(&state.pool).await.map_err(internal)?;
    let mut out = Vec::new();
    for r in reqs {
        let id: String = r.get(0);
        let lines = sqlx::query("SELECT product_id,sku,name,quantity,price_cents FROM request_lines WHERE request_id=? ORDER BY name").bind(&id).fetch_all(&state.pool).await.map_err(internal)?.into_iter().map(|l| json!({"product_id":l.get::<String,_>(0),"sku":l.get::<String,_>(1),"name":l.get::<String,_>(2),"quantity":l.get::<i64,_>(3),"price_cents":l.get::<Option<i64>,_>(4)})).collect::<Vec<_>>();
        out.push(json!({"id":id,"client_name":r.get::<String,_>(1),"company":r.get::<String,_>(2),"email":r.get::<String,_>(3),"po_number":r.get::<String,_>(4),"note":r.get::<String,_>(5),"status":r.get::<String,_>(6),"created_at":r.get::<String,_>(7),"lines":lines}));
    }
    Ok(Json(json!({"requests":out})))
}
fn internal<E: std::fmt::Display>(e: E) -> (StatusCode, Json<Value>) {
    tracing::error!(error=%e,"request failed");
    err(
        StatusCode::INTERNAL_SERVER_ERROR,
        "The server could not finish that action. Try again.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::Request,
    };
    use tower::ServiceExt;
    #[test]
    fn generated_tokens_are_long_and_distinct() {
        let a = token(28);
        let b = token(28);
        assert_eq!(a.len(), 28);
        assert_ne!(a, b);
    }
    #[test]
    fn settings_need_currency() {
        let s = Settings {
            business_name: "Shop".into(),
            price_label: "Price".into(),
            tax_note: "".into(),
            currency: "US".into(),
        };
        assert!(validate_settings(&s).is_err());
    }

    #[test]
    fn stock_counts_are_not_exposed() {
        let product = Product {
            id: "p1".into(),
            sku: "NW-101".into(),
            name: "Notebook".into(),
            description: "Recycled paper".into(),
            category: "Desk".into(),
            price_cents: Some(850),
            stock_note: "Ask about availability".into(),
        };
        let value = serde_json::to_value(product).unwrap();
        assert!(value.get("stock_note").is_some());
        assert!(value.get("stock_count").is_none());
        assert!(value.get("quantity_available").is_none());
    }

    async fn call(
        app: &Router,
        method: &str,
        path: &str,
        body: Value,
        bearer: Option<&str>,
    ) -> (StatusCode, Value) {
        let mut builder = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json");
        if let Some(value) = bearer {
            builder = builder.header("authorization", format!("Bearer {value}"));
        }
        let mut req = builder.body(Body::from(body.to_string())).unwrap();
        req.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 3000))));
        let response = app.clone().oneshot(req).await.unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        let body = if bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap()
        };
        (status, body)
    }

    #[tokio::test]
    async fn owner_to_client_request_flow() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let app = api_router(AppState::new(pool));
        let (status, setup) = call(
            &app,
            "POST",
            "/setup",
            json!({"business_name":"Northline","password":"correct horse battery"}),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let session = setup["token"].as_str().unwrap();
        let product_id = uuid::Uuid::new_v4().to_string();
        let (status,_)=call(&app,"PUT","/admin/catalogue",json!({"settings":{"business_name":"Northline","price_label":"Trade price","tax_note":"Ex VAT","currency":"GBP"},"products":[{"id":product_id,"sku":"A-1","name":"Oak tray","description":"Solid oak","category":"Service","price_cents":null,"stock_note":"Made to order"}]}),Some(session)).await;
        assert_eq!(status, StatusCode::OK);
        let (status, link) = call(
            &app,
            "POST",
            "/admin/links",
            json!({"label":"Juniper Corner"}),
            Some(session),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let link_token = link["token"].as_str().unwrap();
        let (status, catalogue) = call(
            &app,
            "GET",
            &format!("/catalogue/{link_token}"),
            json!(null),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(catalogue["products"][0]["sku"], "A-1");
        let (status,created)=call(&app,"POST",&format!("/requests/{link_token}"),json!({"client_name":"Maya Patel","company":"Juniper Corner","email":"maya@example.test","po_number":"PO-9","note":"Quote delivery","lines":[{"product_id":product_id,"quantity":4}]}),None).await;
        assert_eq!(status, StatusCode::CREATED);
        assert!(created["id"].as_str().unwrap().starts_with("RQ-"));
        let (status, list) = call(&app, "GET", "/admin/requests", json!(null), Some(session)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(list["requests"][0]["lines"][0]["quantity"], 4);
    }

    #[tokio::test]
    async fn write_burst_returns_429_and_retry_after() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let app = api_router(AppState::new(pool));
        let mut last = None;
        for _ in 0..13 {
            let mut req = Request::builder()
                .method("POST")
                .uri("/login")
                .header("content-type", "application/json")
                .body(Body::from("{\"password\":\"wrong password\"}"))
                .unwrap();
            req.extensions_mut()
                .insert(ConnectInfo(SocketAddr::from(([192, 0, 2, 2], 3000))));
            last = Some(app.clone().oneshot(req).await.unwrap());
        }
        let response = last.unwrap();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(response.headers().get(header::RETRY_AFTER).unwrap(), "1");
    }

    #[tokio::test]
    async fn demo_workspaces_are_ephemeral_and_isolated() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let app = api_router(AppState::new(pool));
        let (_, first) = call(&app, "POST", "/demo", json!(null), None).await;
        let (_, second) = call(&app, "POST", "/demo", json!(null), None).await;
        let first_id = first["id"].as_str().unwrap();
        let second_id = second["id"].as_str().unwrap();
        assert_ne!(first_id, second_id);
        let (status, _) = call(
            &app,
            "POST",
            &format!("/demo/{first_id}/requests"),
            json!({"client_name":"Alex Buyer","company":"Test Client","email":"alex@example.test","po_number":"","note":"","lines":[{"product_id":"p1","quantity":2}]}),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        let (_, first_requests) = call(
            &app,
            "GET",
            &format!("/demo/{first_id}/requests"),
            json!(null),
            None,
        )
        .await;
        let (_, second_requests) = call(
            &app,
            "GET",
            &format!("/demo/{second_id}/requests"),
            json!(null),
            None,
        )
        .await;
        assert_eq!(first_requests["requests"].as_array().unwrap().len(), 2);
        assert_eq!(second_requests["requests"].as_array().unwrap().len(), 1);
        let (status, _) = call(
            &app,
            "DELETE",
            &format!("/demo/{first_id}/requests"),
            json!(null),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NO_CONTENT);
    }
}
