use super::service::counter_service;
use super::store::counter_store;
use leptos::prelude::*;

#[component]
pub fn SimpleCounter() -> impl IntoView {
    let value = || counter_store().map(|x| *x);
    let clear = |_| counter_service().clear();
    let decrement = |_| counter_service().decrement();
    let increment = |_| counter_service().increment();

    view! {
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}
