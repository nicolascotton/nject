mod common;
use common::CommonProvider as SharedProvider;
use nject::{injectable, provide, provider};

#[test]
fn provide_with_simple_extended_value_should_provide_value() {
    // Given
    #[provider]
    #[provide(i32, 123)]
    struct SubProvider;

    #[provider]
    struct Provider {
        #[extend]
        sub: SubProvider,
    }
    let prov = Provider { sub: SubProvider };
    // When
    let value = prov.provide::<i32>();
    // Then
    assert_eq!(value, SubProvider.provide::<i32>());
}

#[test]
fn provide_with_extended_value_from_extended_provider_should_provide_value() {
    // Given
    #[provider]
    #[provide(i32, 123)]
    struct SubSubProvider;

    #[provider]
    #[provide(f32, 1.23)]
    struct SubProvider(#[extend] SubSubProvider);

    #[provider]
    struct Provider {
        #[extend]
        sub: SubProvider,
    }
    let prov = Provider {
        sub: SubProvider(SubSubProvider),
    };
    // When
    let ivalue = prov.provide::<i32>();
    let fvalue = prov.provide::<f32>();
    // Then
    assert_eq!(ivalue, SubSubProvider.provide::<i32>());
    assert_eq!(fvalue, SubProvider(SubSubProvider).provide::<f32>());
}

#[test]
fn provide_with_extended_value_in_provider_from_other_module_should_provide_value() {
    // Given
    #[provider]
    struct Provider {
        #[extend]
        sub: common::CommonProvider,
    }
    let prov = Provider {
        sub: common::CommonProvider,
    };
    // When
    let value = prov.provide::<i32>();
    // Then
    assert_eq!(value, common::CommonProvider.provide::<i32>());
}

#[test]
fn provide_with_extended_value_from_provider_with_id_should_provide_value() {
    // Given
    #[provider]
    struct Provider {
        #[extend(CommonProvider)]
        sub: SharedProvider,
    }
    let prov = Provider {
        sub: SharedProvider,
    };
    // When
    let value = prov.provide::<i32>();
    // Then
    assert_eq!(value, common::CommonProvider.provide::<i32>());
}

#[test]
fn provide_with_extended_generic_value_should_provide_value() {
    // Given
    #[provider]
    #[provide(&'a dyn Greeter, self.0)]
    struct SubProvider<'a, T: Greeter>(&'a T);

    #[provider]
    struct Provider<'b, U: Greeter> {
        #[extend]
        sub: SubProvider<'b, U>,
    }
    let prov = Provider {
        sub: SubProvider(&DevGreeter),
    };
    // When
    let value = prov.provide::<&dyn Greeter>().greet();
    // Then
    assert_eq!(
        value,
        SubProvider(&DevGreeter).provide::<&dyn Greeter>().greet()
    );
}

trait Greeter {
    fn greet(&self) -> &'static str;
}

#[injectable]
struct DevGreeter;

impl Greeter for DevGreeter {
    fn greet(&self) -> &'static str {
        "Greeting Dev"
    }
}

#[injectable]
struct ProdGreeter;

impl Greeter for ProdGreeter {
    fn greet(&self) -> &'static str {
        "Greeting production"
    }
}
