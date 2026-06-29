use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use axum_example::CreateUser;
use axum_example::UserService;
use nject::{init, injectable, provider};

#[provider]
#[injectable]
pub struct Provider(#[import] axum_example::Module);

#[tokio::main]
async fn main() {
    let provider: &'static Provider = Box::leak(Box::new(init!()));
    let app = Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/{id}", get(get_user))
        .with_state(provider);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr); // run our app with hyper, listening globally on port 3000
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(prov): State<&'static Provider>,
    Json(user): Json<CreateUser>,
) -> impl IntoResponse {
    let service = prov.provide::<UserService>();
    let user = service.create(user);
    (StatusCode::CREATED, Json(user))
}

async fn get_user(
    State(prov): State<&'static Provider>,
    Path(user_id): Path<usize>,
) -> impl IntoResponse {
    let service = prov.provide::<UserService>();
    let user = service.get(user_id);
    (StatusCode::OK, Json(user))
}
