use actix_example::{Provider, setup_app};
use actix_web::{HttpServer, web::Data};
use sqlx::{Connection, SqliteConnection, query};

const DB_URL: &str = "file::memory:?cache=shared";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database tables
    let mut conn = SqliteConnection::connect(DB_URL)
        .await
        .expect("Unable to connect to database");
    query("CREATE TABLE IF NOT EXISTS user (name TEXT NOT NULL)")
        .execute(&mut conn)
        .await
        .expect("Unable to initialize database");

    let provider = Data::new(Provider::new(DB_URL));
    let addr = ("127.0.0.1", 8080);
    println!("listening on {}:{}", &addr.0, &addr.1);
    HttpServer::new(move || setup_app(provider.clone()))
        .bind(addr)?
        .run()
        .await
}
