use axum::{
    response::Html,
    routing::{delete, get, get_service, post, put},
    Extension, Router,
};

use axum::extract::{DefaultBodyLimit, State};
use common::db::{db_conn, DB};
use sea_orm::DatabaseConnection;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod user;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

pub async fn router() -> Router {
    let state = AppState {
        conn: db_conn().await,
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler))
        // `POST /users` goes to `create_user`
        //.route("/users", post(handler))
        // 无需授权Api.通用模块
        .nest("/comm", no_auth_api())
        .nest("/static", static_web())
        .with_state(state);

    app
}

// 无需授权api
fn no_auth_api() -> Router<AppState> {
    Router::new()
        .route("/login", post(user::user::login)) // 登录
        .route("/searchuser", get(user::user::search_user)) // 搜索
        .route("/from", get(user::user::show_form))
        .route("/do/from", post(user::user::accept_form))
        .route("/upload/:id", get(user::user::upload))
        .route("/do/upload", post(user::user::accept_upload))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
    //.route("/log_out", post(system::log_out)) // 退出登录
}

fn static_web() -> Router<AppState> {
    let path = &config::CONFIG.server.upload_path;
    Router::new().nest_service("/uploads", ServeDir::new(path))
}

fn set_auth_middleware(router: Router) -> Router {
    let router = router;
    //.layer(middleware::from_fn(middleware_fn::ApiAuth))
    //.layer(middleware::from_fn(middleware_fn::Ctx))
    //.layer(middleware::from_extractor::<Claims>());
    router
}

async fn handler(State(state): State<AppState>) -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
