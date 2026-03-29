/// Provider provides Named<key!("foo"), String> but injectable expects named("bar").
use nject::{Named, injectable, key, provider};

#[injectable]
struct Service {
    #[inject(named("bar"))]
    value: String,
}

#[provider]
#[provide(Named<key!("foo"), String>, Named::new("hello".into()))]
struct WrongKeyProvider;

fn main() {
    let provider = WrongKeyProvider;
    let _svc: Service = provider.provide();
}
