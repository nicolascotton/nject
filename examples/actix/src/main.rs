use actix_example::{setup_app, Provider};
use actix_web::HttpServer;
use sqlx::{query, Connection, SqliteConnection};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database tables
    let mut conn = SqliteConnection::connect("file::memory:?cache=shared")
        .await
        .expect("Unable to connect to database");
    query("CREATE TABLE IF NOT EXISTS user (name TEXT NOT NULL)")
        .execute(&mut conn)
        .await
        .expect("Unable to initialize database");

    // Using an Arc to share the provider across multiple threads.
    let provider = Arc::new(Provider::new());
    let addr = ("127.0.0.1", 8080);
    println!("listening on {}:{}", &addr.0, &addr.1);
    HttpServer::new(move || setup_app(provider.clone()))
        .bind(addr)?
        .run()
        .await
}
