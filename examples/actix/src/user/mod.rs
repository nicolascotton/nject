mod errors;
mod models;
mod repository;
mod routes;
mod service;
use nject::{inject, injectable, module};
pub use routes::create_scope;
use std::ops::Deref;

// The options are specied by a provider
pub struct ConnectionOptions<'a> {
    pub url: &'a str,
}

#[derive(Clone)]
struct Pool(sqlx::pool::Pool<sqlx::Sqlite>);

impl Deref for Pool {
    type Target = sqlx::pool::Pool<sqlx::Sqlite>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[injectable]
#[module]
pub struct UserModule {
    #[inject(|o: &'prov ConnectionOptions<'prov>| Pool(sqlx::SqlitePool::connect_lazy(o.url).expect("Invalid database URL")))]
    #[export(Pool, |x| x.clone())]
    pool: Pool,
}
