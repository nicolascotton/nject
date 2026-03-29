/// Provider does not provide Named<key!("db_url"), String>, so injection should fail.
use nject::{injectable, provider};

#[injectable]
struct Service {
    #[inject(named("db_url"))]
    db_url: String,
}

#[provider]
struct EmptyProvider;

fn main() {
    let provider = EmptyProvider;
    let _svc: Service = provider.provide();
}
