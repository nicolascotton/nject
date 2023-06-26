mod counter;
pub use counter::*;
use leptos::*;
use nject::{injectable, provider};

const PROVIDER: *mut Provider = std::ptr::null_mut();

#[provider]
#[provide(&'prov CounterStore, &self.store)]
#[injectable]
pub struct Provider {
    store: CounterStore,
}

impl Provider {
    /// Initialize the Provider in static const PROVIDER.
    pub fn init(cx: Scope) {
        #[provider]
        #[provide(Scope, self.0)]
        struct InitProvider(Scope);

        let init_prov = InitProvider(cx);
        let prov = init_prov.provide::<Provider>();
        unsafe { std::ptr::swap(PROVIDER, Box::leak(Box::from(prov))) };
    }

    /// Injects types providable by static PROVIDER.
    /// The PROVIDER must be initialized before calling inject.
    pub fn inject<'prov, T>() -> T
    where
        Provider: nject::Provider<'prov, T>,
    {
        unsafe { (*PROVIDER).provide() }
    }
}
