mod errors;
mod models;
mod repository;
mod routes;
mod service;
use nject::{injectable, module};
pub use routes::create_scope;

#[injectable]
#[module]
pub struct UserModule {
    #[export]
    service: service::UserService,
}
