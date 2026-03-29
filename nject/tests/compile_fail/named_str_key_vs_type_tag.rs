/// Provider provides Named<key!("db_url"), String> (string key) but
/// injectable expects named(DbUrl) (type tag). These are different types.
use nject::{Named, injectable, key, provider};

struct DbUrl;

#[injectable]
struct Service {
    #[inject(named(DbUrl))]
    value: String,
}

#[provider]
#[provide(Named<key!("db_url"), String>, Named::new("hello".into()))]
struct MismatchedProvider;

fn main() {
    let provider = MismatchedProvider;
    let _svc: Service = provider.provide();
}
