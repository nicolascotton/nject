use super::{counter_service, counter_store};
use leptos::{component, view, IntoView, Scope};

#[component]
pub fn SimpleCounter(cx: Scope) -> impl IntoView {
    let value = || counter_store().map(|x| *x);
    let clear = |_| counter_service().clear();
    let decrement = |_| counter_service().decrement();
    let increment = |_| counter_service().increment();

    view! {
        cx,
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}
