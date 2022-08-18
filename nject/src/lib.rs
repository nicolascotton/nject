pub use nject_macro::{injectable, provider};

pub trait Provider<Value> {
    fn provide(&self) -> Value;
}

pub trait Injectable<Injecty, Provider>
{
    fn inject(provider: &Provider) -> Injecty;
}
