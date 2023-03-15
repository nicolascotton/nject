use leptos::{create_signal, ReadSignal, WriteSignal, Scope, SignalWith, SignalUpdate};
use nject::inject;
use crate::Provider;

#[inject({ 
	let (read, write) = create_signal(cx, 0);
	Self { read, write }
}, cx: Scope)]
pub struct CounterStore {
	read: ReadSignal<i32>,
	write: WriteSignal<i32>
}

impl CounterStore {
	pub fn map<T>(&self, map: impl FnOnce(&i32) -> T) -> T {
		self.read.with(map)
	}

	pub fn update(&self, update: impl FnOnce(&mut i32) ) {
		self.write.update(update);
	}
}

pub fn counter_store<'a>() -> &'a CounterStore {
	Provider::inject::<&CounterStore>()
}