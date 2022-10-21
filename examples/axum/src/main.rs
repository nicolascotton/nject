use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use models::CreateUser;
use nject::{provide, provider};
use repository::{memory::MemoryRepository, Repository};
use service::UserService;
use std::{net::SocketAddr, sync::Arc};

mod models;
mod repository;
mod service;

#[provider]
#[provide(&'prov dyn Repository, &self.repository)]
pub struct Provider {
    repository: MemoryRepository,
}

#[tokio::main]
async fn main() {
    let app = Router::with_state(Arc::new(Provider {
        repository: MemoryRepository::new(),
    }))
    .route("/api/users", post(create_user))
    .route("/api/users/:id", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_user(
    State(prov): State<Arc<Provider>>,
    Json(user): Json<CreateUser>,
) -> impl IntoResponse {
    let service = prov.provide::<UserService>();
    let user = service.create(user);
    (StatusCode::CREATED, Json(user))
}

async fn get_user(
    State(prov): State<Arc<Provider>>,
    Path(user_id): Path<usize>,
) -> impl IntoResponse {
    let service = prov.provide::<UserService>();
    let user = service.get(user_id);
    (StatusCode::OK, Json(user))
}
