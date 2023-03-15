use leptos::*;
use leptos_example::*;

pub fn main() {
    mount_to_body(|cx| {
        // init must be called in the root scope before any injections.
        Provider::init(cx);
        view! { cx,  <SimpleCounter /> }
    })
}
