# Leptos
Follow the instructions provided in the [Leptos](https://github.com/leptos-rs/leptos) project to run the application.

## Alternative solution
If you want a provider scoped from the component instead of a static one, you should use the `provide_context` & `use_context` functions:
```rust
provide_context(Rc::new(Provider::new()); // To insert the provider into the scope
let provider = use_context::<Rc<Provider>>().unwrap(); // To access the provider under the scope
```