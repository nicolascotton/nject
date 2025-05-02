mod counter;
pub use counter::*;
use nject::{injectable, provider};

const PROVIDER: *mut Provider = std::ptr::null_mut();

#[injectable]
#[provider]
pub struct Provider {
    #[import]
    counter_mod: CounterModule,
}

impl Provider {
    /// Initialize the Provider in static const PROVIDER.
    pub fn init() {
        #[provider]
        struct InitProvider;

        let prov = InitProvider.provide::<Provider>();
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
