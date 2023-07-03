mod component;
mod service;
mod store;
pub use component::*;
use nject::{module, injectable};

#[injectable]
#[module]
pub struct CounterModule {
    #[export]
    store: store::CounterStore,
}
