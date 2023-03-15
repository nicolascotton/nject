use super::CounterStore;
use crate::Provider;
use nject::injectable;

#[injectable]
pub struct CounterService<'a> {
    store: &'a CounterStore,
}

impl<'a> CounterService<'a> {
    pub fn clear(&self) {
        self.store.update(|x| *x = 0);
    }
    pub fn decrement(&self) {
        self.store.update(|x| *x -= 1);
    }
    pub fn increment(&self) {
        self.store.update(|x| *x += 1);
    }
}

pub fn counter_service<'a>() -> CounterService<'a> {
    Provider::inject::<CounterService>()
}
