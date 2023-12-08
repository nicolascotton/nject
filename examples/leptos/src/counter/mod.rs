mod component;
mod service;
mod store;
pub use component::SimpleCounter;
use nject::{injectable, module};

#[injectable]
#[module]
pub struct CounterModule {
    #[export]
    store: store::CounterStore,
}
