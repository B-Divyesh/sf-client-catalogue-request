use axum::{
    extract::{ConnectInfo, DefaultBodyLimit, Path, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use dashmap::DashMap;
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
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
const TENANT: &str = "35c6fe40-0ec0-46b6-98c6-213ad4de6650";
const CLIENT: &str = "25c704f4-465a-47af-80ab-2c489466b697";
const API_SCOPE: &str = "access_as_user";
const DISCOVERY:&str="https://sociobotcustomers.ciamlogin.com/35c6fe40-0ec0-46b6-98c6-213ad4de6650/v2.0/.well-known/openid-configuration";
const VERIFY_BASE: &str =
    "https://api.sociobot.in/api/v1/products/client-catalogue-request/verify?license=";
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    rates: Arc<DashMap<String, (Instant, u32)>>,
    http: reqwest::Client,
    auth: Arc<tokio::sync::RwLock<Option<Keys>>>,
    verify_base: String,
}
#[derive(Clone)]
struct Keys {
    issuer: String,
    keys: JwkSet,
    at: Instant,
}
#[derive(Deserialize)]
struct Discovery {
    issuer: String,
    jwks_uri: String,
}
#[derive(Deserialize)]
struct Claims {
    oid: String,
    tid: String,
    #[serde(default)]
    scp: String,
}
#[derive(Deserialize)]
struct Verdict {
    valid: bool,
}
impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            rates: Arc::new(DashMap::new()),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(8))
                .build()
                .expect("HTTP client"),
            auth: Arc::new(tokio::sync::RwLock::new(None)),
            verify_base: std::env::var("SOCIOBOT_LICENSE_VERIFY_BASE")
                .unwrap_or_else(|_| VERIFY_BASE.into()),
        }
    }

    #[cfg(test)]
    fn with_verify_base(pool: SqlitePool, verify_base: String) -> Self {
        let mut state = Self::new(pool);
        state.verify_base = verify_base;
        state
    }
}
type R<T> = Result<T, (StatusCode, Json<Value>)>;
fn err(c: StatusCode, m: &str) -> (StatusCode, Json<Value>) {
    (c, Json(json!({"error":m})))
}
fn token(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
pub fn api_router(state: AppState) -> Router {
    Router::new().route("/auth/config",get(||async{Json(json!({"authority":"https://sociobotcustomers.ciamlogin.com/35c6fe40-0ec0-46b6-98c6-213ad4de6650/","client_id":CLIENT,"scope":format!("api://{CLIENT}/{API_SCOPE}")}))})).route("/admin/catalogue",get(admin_catalogue).put(save_catalogue)).route("/admin/links",post(create_link)).route("/admin/links/{token}/revoke",post(revoke_link)).route("/admin/requests",get(list_requests)).route("/admin/requests/{id}",axum::routing::delete(delete_request)).route("/catalogue/{token}",get(client_catalogue)).route("/requests/{token}",post(create_request)).route("/demo",post(create_demo)).route("/demo/{id}/requests",get(demo_requests).post(create_demo_request).delete(delete_demo)).layer(DefaultBodyLimit::max(5*1024*1024)).layer(middleware::from_fn_with_state(state.clone(),limit)).with_state(state)
}
async fn limit(
    State(s): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    h: HeaderMap,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let ip = h
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|| peer.ip().to_string());
    let now = Instant::now();
    let mut x = s.rates.entry(ip).or_insert((now, 0));
    if now.duration_since(x.0) >= Duration::from_secs(1) {
        *x = (now, 0)
    }
    x.1 += 1;
    let max = if matches!(
        *req.method(),
        axum::http::Method::POST | axum::http::Method::PUT | axum::http::Method::DELETE
    ) {
        12
    } else {
        40
    };
    if x.1 > max {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, HeaderValue::from_static("1"))],
            Json(json!({"error":"Too many requests. Wait one second and try again."})),
        )
            .into_response();
    }
    drop(x);
    next.run(req).await
}
async fn keys(s: &AppState, refresh: bool) -> R<Keys> {
    if !refresh {
        if let Some(v) = s.auth.read().await.clone() {
            if v.at.elapsed() < Duration::from_secs(3600) {
                return Ok(v);
            }
        }
    }
    let d: Discovery = s
        .http
        .get(DISCOVERY)
        .send()
        .await
        .map_err(internal)?
        .error_for_status()
        .map_err(internal)?
        .json()
        .await
        .map_err(internal)?;
    let keys = s
        .http
        .get(d.jwks_uri)
        .send()
        .await
        .map_err(internal)?
        .error_for_status()
        .map_err(internal)?
        .json()
        .await
        .map_err(internal)?;
    let v = Keys {
        issuer: d.issuer,
        keys,
        at: Instant::now(),
    };
    *s.auth.write().await = Some(v.clone());
    Ok(v)
}
async fn seller(h: &HeaderMap, s: &AppState) -> R<String> {
    #[cfg(debug_assertions)]
    if let Some(v) = h.get("x-test-seller").and_then(|v| v.to_str().ok()) {
        return Ok(v.into());
    }
    let raw = h
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(unauth)?;
    #[cfg(debug_assertions)]
    if let Some(subject) = raw.strip_prefix("test-seller:") {
        if !subject.is_empty() {
            return Ok(subject.into());
        }
    }
    let kid = decode_header(raw)
        .map_err(|_| unauth())?
        .kid
        .ok_or_else(unauth)?;
    let mut k = keys(s, false).await?;
    if !k
        .keys
        .keys
        .iter()
        .any(|x| x.common.key_id.as_deref() == Some(&kid))
    {
        k = keys(s, true).await?
    }
    let jwk = k
        .keys
        .keys
        .iter()
        .find(|x| x.common.key_id.as_deref() == Some(&kid))
        .ok_or_else(unauth)?;
    let key = DecodingKey::from_jwk(jwk).map_err(|_| unauth())?;
    let mut v = Validation::new(Algorithm::RS256);
    v.set_audience(&[CLIENT]);
    v.set_issuer(&[k.issuer]);
    let c = decode::<Claims>(raw, &key, &v)
        .map_err(|_| unauth())?
        .claims;
    if c.tid != TENANT || c.oid.is_empty() || !has_api_scope(&c.scp) {
        return Err(unauth());
    }
    Ok(c.oid)
}
fn has_api_scope(scopes: &str) -> bool {
    scopes
        .split_ascii_whitespace()
        .any(|scope| scope == API_SCOPE)
}
fn unauth() -> (StatusCode, Json<Value>) {
    err(
        StatusCode::UNAUTHORIZED,
        "Sign in with Sociobot to open the seller workspace.",
    )
}
async fn ensure(pool: &SqlitePool, id: &str) -> R<()> {
    sqlx::query("INSERT OR IGNORE INTO sellers(subject) VALUES(?)")
        .bind(id)
        .execute(pool)
        .await
        .map_err(internal)?;
    sqlx::query("INSERT OR IGNORE INTO tenant_settings(seller_subject) VALUES(?)")
        .bind(id)
        .execute(pool)
        .await
        .map_err(internal)?;
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
struct ClientLink {
    token: String,
    label: String,
    active: bool,
}
#[derive(Serialize, Deserialize)]
struct Catalogue {
    settings: Settings,
    products: Vec<Product>,
    links: Option<Vec<ClientLink>>,
}
#[derive(Deserialize)]
struct Save {
    settings: Settings,
    products: Vec<Product>,
}
#[derive(Deserialize)]
struct Link {
    label: String,
}
async fn admin_catalogue(State(s): State<AppState>, h: HeaderMap) -> R<Json<Catalogue>> {
    let id = seller(&h, &s).await?;
    ensure(&s.pool, &id).await?;
    Ok(Json(load(&s.pool, &id, true).await?))
}
fn valid_settings(x: &Settings) -> R<()> {
    if x.business_name.trim().is_empty()
        || x.business_name.len() > 80
        || x.price_label.trim().is_empty()
        || x.price_label.len() > 40
        || x.tax_note.len() > 160
        || x.currency.trim().len() != 3
    {
        Err(err(
            StatusCode::BAD_REQUEST,
            "Add a business name, price label, and three-letter currency code.",
        ))
    } else {
        Ok(())
    }
}
fn valid_product(p: &Product, n: usize) -> R<()> {
    if p.sku.trim().is_empty()
        || p.name.trim().is_empty()
        || p.sku.len() > 80
        || p.name.len() > 120
        || p.description.len() > 1000
        || p.category.len() > 80
        || p.stock_note.len() > 200
        || p.price_cents.is_some_and(|v| v < 0)
    {
        Err(err(
            StatusCode::BAD_REQUEST,
            &format!(
                "Row {} needs a SKU, a short name, and a valid price or POA.",
                n + 1
            ),
        ))
    } else {
        Ok(())
    }
}
async fn paid(h: &HeaderMap, s: &AppState) -> R<bool> {
    let Some(v) = h
        .get("x-sociobot-license")
        .and_then(|v| v.to_str().ok())
        .filter(|v| !v.is_empty())
    else {
        return Ok(false);
    };
    let u = format!("{}{}", s.verify_base, urlencoding::encode(v));
    let x = s
        .http
        .get(u)
        .send()
        .await
        .map_err(|_| {
            err(
                StatusCode::SERVICE_UNAVAILABLE,
                "Could not check the license. Try again when connected.",
            )
        })?
        .error_for_status()
        .map_err(|_| {
            err(
                StatusCode::SERVICE_UNAVAILABLE,
                "Could not check the license. Try again when connected.",
            )
        })?
        .json::<Verdict>()
        .await
        .map_err(internal)?;
    Ok(x.valid)
}
async fn save_catalogue(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(x): Json<Save>,
) -> R<Json<Value>> {
    let id = seller(&h, &s).await?;
    ensure(&s.pool, &id).await?;
    if x.products.len() > 5000 {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "This import has more than 5,000 rows. Split it into smaller files.",
        ));
    }
    valid_settings(&x.settings)?;
    if x.products.len() > 12 && !paid(&h, &s).await? {
        return Err(err(
            StatusCode::FORBIDDEN,
            "The free workspace includes 12 catalogue rows. Add an active license to import more.",
        ));
    }
    let mut tx = s.pool.begin().await.map_err(internal)?;
    sqlx::query("UPDATE tenant_settings SET business_name=?,price_label=?,tax_note=?,currency=? WHERE seller_subject=?").bind(x.settings.business_name.trim()).bind(x.settings.price_label.trim()).bind(x.settings.tax_note.trim()).bind(x.settings.currency.trim().to_uppercase()).bind(&id).execute(&mut *tx).await.map_err(internal)?;
    sqlx::query("DELETE FROM tenant_products WHERE seller_subject=?")
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(internal)?;
    for (n, p) in x.products.iter().enumerate() {
        valid_product(p, n)?;
        sqlx::query("INSERT INTO tenant_products(seller_subject,id,sku,name,description,category,price_cents,stock_note) VALUES(?,?,?,?,?,?,?,?)").bind(&id).bind(if p.id.is_empty(){uuid::Uuid::new_v4().to_string()}else{p.id.clone()}).bind(p.sku.trim()).bind(p.name.trim()).bind(p.description.trim()).bind(p.category.trim()).bind(p.price_cents).bind(p.stock_note.trim()).execute(&mut *tx).await.map_err(|_|err(StatusCode::BAD_REQUEST,&format!("SKU {} appears more than once.",p.sku)))?;
    }
    tx.commit().await.map_err(internal)?;
    Ok(Json(json!({"saved":true,"count":x.products.len()})))
}
async fn create_link(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(x): Json<Link>,
) -> R<Json<ClientLink>> {
    let id = seller(&h, &s).await?;
    ensure(&s.pool, &id).await?;
    if x.label.trim().len() < 2 || x.label.len() > 120 {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Name the client or client group for this link.",
        ));
    }
    let n: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM tenant_links WHERE seller_subject=? AND active=1")
            .bind(&id)
            .fetch_one(&s.pool)
            .await
            .map_err(internal)?;
    if n >= 1 && !paid(&h, &s).await? {
        return Err(err(
            StatusCode::FORBIDDEN,
            "The free workspace includes one client link. Add an active license to create more.",
        ));
    }
    let l = ClientLink {
        token: token(28),
        label: x.label.trim().into(),
        active: true,
    };
    sqlx::query("INSERT INTO tenant_links(token,seller_subject,label) VALUES(?,?,?)")
        .bind(&l.token)
        .bind(id)
        .bind(&l.label)
        .execute(&s.pool)
        .await
        .map_err(internal)?;
    Ok(Json(l))
}
async fn revoke_link(
    State(s): State<AppState>,
    h: HeaderMap,
    Path(t): Path<String>,
) -> R<StatusCode> {
    let id = seller(&h, &s).await?;
    let n=sqlx::query("UPDATE tenant_links SET active=0,revoked_at=? WHERE token=? AND seller_subject=? AND active=1").bind(Utc::now().to_rfc3339()).bind(t).bind(id).execute(&s.pool).await.map_err(internal)?.rows_affected();
    if n == 0 {
        Err(err(
            StatusCode::NOT_FOUND,
            "That client link is already inactive.",
        ))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
async fn load(pool: &SqlitePool, id: &str, links: bool) -> R<Catalogue> {
    let s=sqlx::query("SELECT business_name,price_label,tax_note,currency FROM tenant_settings WHERE seller_subject=?").bind(id).fetch_one(pool).await.map_err(internal)?;
    let products=sqlx::query("SELECT id,sku,name,description,category,price_cents,stock_note FROM tenant_products WHERE seller_subject=? AND active=1 ORDER BY category,name").bind(id).fetch_all(pool).await.map_err(internal)?.into_iter().map(|r|Product{id:r.get(0),sku:r.get(1),name:r.get(2),description:r.get(3),category:r.get(4),price_cents:r.get(5),stock_note:r.get(6)}).collect();
    let links = if links {
        Some(sqlx::query("SELECT token,label,active FROM tenant_links WHERE seller_subject=? ORDER BY created_at DESC").bind(id).fetch_all(pool).await.map_err(internal)?.into_iter().map(|r|ClientLink{token:r.get(0),label:r.get(1),active:r.get::<i64,_>(2)==1}).collect())
    } else {
        None
    };
    Ok(Catalogue {
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
async fn client_catalogue(State(s): State<AppState>, Path(t): Path<String>) -> R<Json<Catalogue>> {
    let id: Option<String> =
        sqlx::query_scalar("SELECT seller_subject FROM tenant_links WHERE token=? AND active=1")
            .bind(&t)
            .fetch_optional(&s.pool)
            .await
            .map_err(internal)?;
    let id = id.ok_or_else(|| {
        err(
            StatusCode::NOT_FOUND,
            "This client link is not active. Ask the seller for a new link.",
        )
    })?;
    Ok(Json(load(&s.pool, &id, false).await?))
}
#[derive(Deserialize)]
struct RequestInput {
    client_name: String,
    company: String,
    email: String,
    po_number: String,
    note: String,
    lines: Vec<RequestLine>,
}
#[derive(Deserialize)]
struct RequestLine {
    product_id: String,
    quantity: i64,
}
fn valid_request(x: &RequestInput) -> R<()> {
    if x.client_name.trim().len() < 2
        || x.company.trim().len() < 2
        || !x.email.contains('@')
        || x.email.len() > 254
        || x.po_number.len() > 80
        || x.note.len() > 2000
    {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Add your name, company, and a valid email address.",
        ));
    }
    if x.lines.is_empty()
        || x.lines.len() > 100
        || x.lines.iter().any(|l| l.quantity < 1 || l.quantity > 9999)
    {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Add at least one product. Each quantity must be from 1 to 9,999.",
        ));
    }
    let mut ids = HashSet::new();
    if x.lines.iter().any(|l| !ids.insert(&l.product_id)) {
        return Err(err(
            StatusCode::BAD_REQUEST,
            "Each product can appear only once. Change its quantity instead.",
        ));
    }
    Ok(())
}
async fn create_request(
    State(s): State<AppState>,
    Path(t): Path<String>,
    Json(x): Json<RequestInput>,
) -> R<(StatusCode, Json<Value>)> {
    let owner: Option<String> =
        sqlx::query_scalar("SELECT seller_subject FROM tenant_links WHERE token=? AND active=1")
            .bind(&t)
            .fetch_optional(&s.pool)
            .await
            .map_err(internal)?;
    let owner = owner.ok_or_else(|| {
        err(
            StatusCode::NOT_FOUND,
            "This client link is not active. Ask the seller for a new link.",
        )
    })?;
    valid_request(&x)?;
    let id = format!(
        "RQ-{}",
        uuid::Uuid::new_v4().simple().to_string()[..8].to_uppercase()
    );
    let mut tx = s.pool.begin().await.map_err(internal)?;
    sqlx::query("INSERT INTO tenant_requests(id,seller_subject,link_token,client_name,company,email,po_number,note) VALUES(?,?,?,?,?,?,?,?)").bind(&id).bind(&owner).bind(&t).bind(x.client_name.trim()).bind(x.company.trim()).bind(x.email.trim()).bind(x.po_number.trim()).bind(x.note.trim()).execute(&mut *tx).await.map_err(internal)?;
    for l in x.lines {
        let p=sqlx::query("SELECT sku,name,price_cents FROM tenant_products WHERE seller_subject=? AND id=? AND active=1").bind(&owner).bind(&l.product_id).fetch_optional(&mut *tx).await.map_err(internal)?.ok_or_else(||err(StatusCode::BAD_REQUEST,"A selected product is no longer available. Reload the catalogue."))?;
        sqlx::query("INSERT INTO tenant_request_lines(request_id,product_id,sku,name,quantity,price_cents) VALUES(?,?,?,?,?,?)").bind(&id).bind(l.product_id).bind(p.get::<String,_>(0)).bind(p.get::<String,_>(1)).bind(l.quantity).bind(p.get::<Option<i64>,_>(2)).execute(&mut *tx).await.map_err(internal)?;
    }
    tx.commit().await.map_err(internal)?;
    Ok((StatusCode::CREATED, Json(json!({"id":id}))))
}
async fn list_requests(State(s): State<AppState>, h: HeaderMap) -> R<Json<Value>> {
    let id = seller(&h, &s).await?;
    let rows=sqlx::query("SELECT id,client_name,company,email,po_number,note,status,created_at FROM tenant_requests WHERE seller_subject=? AND deleted_at IS NULL ORDER BY created_at DESC").bind(id).fetch_all(&s.pool).await.map_err(internal)?;
    let mut out = vec![];
    for r in rows {
        let id: String = r.get(0);
        let lines=sqlx::query("SELECT product_id,sku,name,quantity,price_cents FROM tenant_request_lines WHERE request_id=?").bind(&id).fetch_all(&s.pool).await.map_err(internal)?.into_iter().map(|l|json!({"product_id":l.get::<String,_>(0),"sku":l.get::<String,_>(1),"name":l.get::<String,_>(2),"quantity":l.get::<i64,_>(3),"price_cents":l.get::<Option<i64>,_>(4)})).collect::<Vec<_>>();
        out.push(json!({"id":id,"client_name":r.get::<String,_>(1),"company":r.get::<String,_>(2),"email":r.get::<String,_>(3),"po_number":r.get::<String,_>(4),"note":r.get::<String,_>(5),"status":r.get::<String,_>(6),"created_at":r.get::<String,_>(7),"lines":lines}))
    }
    Ok(Json(json!({"requests":out})))
}
async fn delete_request(
    State(s): State<AppState>,
    h: HeaderMap,
    Path(id): Path<String>,
) -> R<StatusCode> {
    let owner = seller(&h, &s).await?;
    let n=sqlx::query("UPDATE tenant_requests SET deleted_at=? WHERE id=? AND seller_subject=? AND deleted_at IS NULL").bind(Utc::now().to_rfc3339()).bind(id).bind(owner).execute(&s.pool).await.map_err(internal)?.rows_affected();
    if n == 0 {
        Err(err(
            StatusCode::NOT_FOUND,
            "That request is already deleted.",
        ))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
fn sample() -> Value {
    json!({"id":"RQ-6C24A19E","client_name":"Maya Patel","company":"Juniper Corner","email":"maya@example.test","po_number":"PO-1842","note":"Please quote delivery to Bristol.","status":"New","created_at":"2026-08-28 09:14","lines":[{"product_id":"p1","sku":"NW-101","name":"Recycled counter notebook","quantity":24,"price_cents":850},{"product_id":"p4","sku":"PK-228","name":"Custom paper tape","quantity":12,"price_cents":null}]})
}

// Demo workspaces are deliberately stateless. The browser keeps its sample
// request list under a `demo:` key, so a request can never land on one
// replica and be read from another, nor can it be retained by this server.
fn demo_ok(id: &str) -> R<()> {
    if id.len() == 32 && id.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(())
    } else {
        Err(err(
            StatusCode::NOT_FOUND,
            "This sample workspace is not available. Reset the demo to start again.",
        ))
    }
}
async fn create_demo() -> R<(StatusCode, Json<Value>)> {
    Ok((StatusCode::CREATED, Json(json!({"id":token(32)}))))
}
async fn demo_requests(Path(id): Path<String>) -> R<Json<Value>> {
    demo_ok(&id)?;
    Ok(Json(json!({"requests":[sample()]})))
}
async fn delete_demo(Path(id): Path<String>) -> StatusCode {
    let _ = demo_ok(&id);
    StatusCode::NO_CONTENT
}
async fn create_demo_request(
    Path(id): Path<String>,
    Json(x): Json<RequestInput>,
) -> R<(StatusCode, Json<Value>)> {
    demo_ok(&id)?;
    valid_request(&x)?;
    let ps = [
        ("p1", "NW-101", "Recycled counter notebook", Some(850)),
        ("p2", "NW-114", "Brass desk ruler", Some(1250)),
        ("p3", "PK-220", "Kraft dispatch box", Some(1890)),
        ("p4", "PK-228", "Custom paper tape", None),
        ("p5", "SV-410", "Shelf label set", Some(2400)),
        ("p6", "SV-421", "Oak display riser", None),
    ];
    let mut ls = vec![];
    for l in &x.lines {
        let p = ps.iter().find(|p| p.0 == l.product_id).ok_or_else(|| {
            err(
                StatusCode::BAD_REQUEST,
                "A selected sample product is not available. Reset the demo.",
            )
        })?;
        ls.push(
            json!({"product_id":p.0,"sku":p.1,"name":p.2,"quantity":l.quantity,"price_cents":p.3}),
        )
    }
    let rid = format!(
        "RQ-DEMO-{}",
        uuid::Uuid::new_v4().simple().to_string()[..4].to_uppercase()
    );
    let request = json!({"id":rid,"client_name":x.client_name.trim(),"company":x.company.trim(),"email":x.email.trim(),"po_number":x.po_number.trim(),"note":x.note.trim(),"status":"New","created_at":Utc::now().to_rfc3339(),"lines":ls});
    Ok((
        StatusCode::CREATED,
        Json(json!({"id":request["id"],"request":request})),
    ))
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

    #[test]
    fn generated_tokens_are_long_and_distinct() {
        assert_eq!(token(28).len(), 28);
        assert_ne!(token(28), token(28));
    }

    #[test]
    fn stock_counts_are_not_exposed() {
        assert!(serde_json::to_value(Product {
            id: "p".into(),
            sku: "s".into(),
            name: "n".into(),
            description: "".into(),
            category: "".into(),
            price_cents: None,
            stock_note: "".into(),
        })
        .unwrap()
        .get("stock_count")
        .is_none());
    }

    #[test]
    fn seller_tokens_require_the_product_api_scope() {
        assert!(has_api_scope("openid access_as_user profile"));
        assert!(!has_api_scope("openid profile email"));
        assert!(!has_api_scope("access_as_user_extra"));
    }

    #[tokio::test]
    async fn demo_is_stateless_across_backend_instances() {
        let (_, created) = create_demo().await.unwrap();
        let id = created["id"].as_str().unwrap().to_string();
        // This does not share AppState or a database connection with creation.
        let response = demo_requests(Path(id)).await.unwrap();
        assert_eq!(response["requests"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn sellers_are_isolated() {
        let p = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!().run(&p).await.unwrap();
        ensure(&p, "a").await.unwrap();
        ensure(&p, "b").await.unwrap();
        sqlx::query(
            "INSERT INTO tenant_products(seller_subject,id,sku,name) VALUES('a','x','A','A')",
        )
        .execute(&p)
        .await
        .unwrap();
        assert_eq!(load(&p, "a", false).await.unwrap().products.len(), 1);
        assert!(load(&p, "b", false).await.unwrap().products.is_empty());
    }

    #[tokio::test]
    async fn free_workspace_cannot_save_thirteen_rows_without_a_verified_license() {
        let p = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!().run(&p).await.unwrap();
        let mut h = HeaderMap::new();
        h.insert("x-test-seller", HeaderValue::from_static("seller-a"));
        let products = (0..13)
            .map(|n| Product {
                id: n.to_string(),
                sku: format!("SKU-{n}"),
                name: format!("Product {n}"),
                description: "".into(),
                category: "Products".into(),
                price_cents: Some(100),
                stock_note: "Ask".into(),
            })
            .collect();
        let result = save_catalogue(
            State(AppState::new(p)),
            h,
            Json(Save {
                settings: Settings {
                    business_name: "Seller".into(),
                    price_label: "Trade price".into(),
                    tax_note: "".into(),
                    currency: "GBP".into(),
                },
                products,
            }),
        )
        .await;
        assert!(matches!(result, Err((StatusCode::FORBIDDEN, _))));
    }

    #[tokio::test]
    async fn verified_license_raises_catalogue_and_link_limits() {
        let fixture = Router::new().route(
            "/verify",
            get(|| async { Json(json!({"valid":true,"reason":"ok","expires_at":null})) }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let fixture_task = tokio::spawn(async move {
            axum::serve(listener, fixture).await.unwrap();
        });

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let state = AppState::with_verify_base(pool, format!("http://{address}/verify?license="));
        let mut headers = HeaderMap::new();
        headers.insert("x-test-seller", HeaderValue::from_static("paid-seller"));
        headers.insert(
            "x-sociobot-license",
            HeaderValue::from_static("recorded-valid-license"),
        );
        let products = (0..13)
            .map(|n| Product {
                id: n.to_string(),
                sku: format!("SKU-{n}"),
                name: format!("Product {n}"),
                description: String::new(),
                category: "Products".into(),
                price_cents: Some(100),
                stock_note: "Ask".into(),
            })
            .collect();
        let saved = save_catalogue(
            State(state.clone()),
            headers.clone(),
            Json(Save {
                settings: Settings {
                    business_name: "Paid Seller".into(),
                    price_label: "Trade price".into(),
                    tax_note: String::new(),
                    currency: "GBP".into(),
                },
                products,
            }),
        )
        .await
        .unwrap();
        assert_eq!(saved["count"], 13);

        let _ = create_link(
            State(state.clone()),
            headers.clone(),
            Json(Link {
                label: "Buyer one".into(),
            }),
        )
        .await
        .unwrap();
        let _ = create_link(
            State(state),
            headers,
            Json(Link {
                label: "Buyer two".into(),
            }),
        )
        .await
        .unwrap();
        fixture_task.abort();
    }

    #[tokio::test]
    async fn catalogue_import_cap_is_five_thousand_rows() {
        let p = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!().run(&p).await.unwrap();
        let mut h = HeaderMap::new();
        h.insert("x-test-seller", HeaderValue::from_static("seller-b"));
        let too_many = (0..5001)
            .map(|n| Product {
                id: n.to_string(),
                sku: format!("SKU-{n}"),
                name: "Product".into(),
                description: "".into(),
                category: "Products".into(),
                price_cents: None,
                stock_note: "Ask".into(),
            })
            .collect::<Vec<_>>();
        let result = save_catalogue(
            State(AppState::new(p)),
            h,
            Json(Save {
                settings: Settings {
                    business_name: "Seller".into(),
                    price_label: "Trade price".into(),
                    tax_note: "".into(),
                    currency: "GBP".into(),
                },
                products: too_many,
            }),
        )
        .await;
        assert!(matches!(result, Err((StatusCode::BAD_REQUEST, _))));
    }

    #[tokio::test]
    async fn revoked_client_links_stop_serving_catalogues() {
        let p = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!().run(&p).await.unwrap();
        let state = AppState::new(p);
        let mut h = HeaderMap::new();
        h.insert("x-test-seller", HeaderValue::from_static("seller-c"));
        let link = create_link(
            State(state.clone()),
            h.clone(),
            Json(Link {
                label: "Buyer".into(),
            }),
        )
        .await
        .unwrap()
        .0;
        assert!(
            client_catalogue(State(state.clone()), Path(link.token.clone()))
                .await
                .is_ok()
        );
        revoke_link(State(state.clone()), h, Path(link.token.clone()))
            .await
            .unwrap();
        assert!(matches!(
            client_catalogue(State(state), Path(link.token)).await,
            Err((StatusCode::NOT_FOUND, _))
        ));
    }
}
