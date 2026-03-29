/// Provider provides Named<Foo, String> but injectable expects named(Bar).
use nject::{Named, injectable, provider};

struct Foo;
struct Bar;

#[injectable]
struct Service {
    #[inject(named(Bar))]
    value: String,
}

#[provider]
#[provide(Named<Foo, String>, Named::new("hello".into()))]
struct WrongTagProvider;

fn main() {
    let provider = WrongTagProvider;
    let _svc: Service = provider.provide();
}
