//! Example: Simplifying provider initialisation with `init!`
//!
//! Two forms are available:
//! - Expression: `let p: Provider = init!(M1, M2);` — for modules with owned exports
//! - Block: `init! { let p: Provider = M1, M2; }` — also supports borrowed exports

use nject::{init, injectable, module, provider};

// --- Module definitions ---

/// A configuration module that exports a database port number (owned).
#[injectable]
#[module]
#[export(u16, 5432)]
struct DbConfigModule;

/// A formatting module that exports a connection string (owned).
/// It depends on `u16` (the port) exported by `DbConfigModule`.
#[injectable]
#[module]
#[export(String, |port: u16| format!("postgres://localhost:{}", port))]
struct ConnectionModule;

// --- An injectable service that uses the connection string ---

#[injectable]
struct DbService {
    connection_string: String,
}

// --- Modules with field-level exports (references) ---

#[derive(Debug)]
#[injectable]
struct Secret(#[inject(42)] i32);

#[injectable]
#[module]
struct SecretModule {
    #[export]
    secret: Secret,
}

#[injectable]
struct SecretConsumer<'a>(&'a Secret);

// --- Providers ---

#[injectable]
#[provider]
struct AppProvider(#[import] DbConfigModule, #[import] ConnectionModule);

#[injectable]
#[provider]
struct SecretProvider(#[import] SecretModule);

fn main() {
    // Works great when module exports are owned values (struct-level #[export]).
    let provider: AppProvider = init!(DbConfigModule, ConnectionModule);

    let svc: DbService = provider.provide();
    println!("[expr]  Connection: {}", svc.connection_string);
    assert_eq!(svc.connection_string, "postgres://localhost:5432");

    // Required when modules use field-level #[export] (which produces references).
    // Supports multiple declarations, like lazy_static!.
    init! {
        let secret_provider: SecretProvider = SecretModule;
    }

    let consumer: SecretConsumer = secret_provider.provide();
    println!("[block] Secret value: {}", consumer.0.0);
    assert_eq!(consumer.0.0, 42);
}
