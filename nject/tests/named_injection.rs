use nject::{Key, Named, injectable, key, provider, str_key_hash};

// --- Tag types (zero-sized, zero-cost markers) ---

struct DbUrl;
struct ApiKey;
struct Primary;
struct Secondary;

#[test]
fn named_type_is_zero_cost() {
    // Named<Tag, T> should have the exact same size as T
    assert_eq!(
        core::mem::size_of::<Named<DbUrl, String>>(),
        core::mem::size_of::<String>()
    );
    assert_eq!(
        core::mem::size_of::<Named<ApiKey, i32>>(),
        core::mem::size_of::<i32>()
    );
    // Zero-sized tags should not add any alignment padding
    assert_eq!(
        core::mem::align_of::<Named<DbUrl, String>>(),
        core::mem::align_of::<String>()
    );
}

#[test]
fn named_deref_and_into_inner() {
    let named: Named<DbUrl, String> = Named::new("hello".into());
    // Deref
    assert_eq!(&*named, "hello");
    // into_inner
    assert_eq!(named.into_inner(), "hello");
}

#[test]
fn named_from_value() {
    let named: Named<DbUrl, i32> = Named::from(42);
    assert_eq!(*named, 42);
}

#[test]
fn named_clone_and_eq() {
    let a: Named<DbUrl, i32> = Named::new(42);
    let b = a.clone();
    assert_eq!(a, b);
}

#[injectable]
#[derive(Debug, PartialEq)]
struct ServiceWithDirectNamed(Named<DbUrl, String>, Named<ApiKey, String>);

#[test]
fn provide_direct_named_unnamed_struct() {
    // Given
    #[provider]
    #[provide(Named<DbUrl, String>, Named::new("postgres://localhost".into()))]
    #[provide(Named<ApiKey, String>, Named::new("secret-key".into()))]
    struct DirectProvider;

    let provider = DirectProvider;
    // When
    let svc: ServiceWithDirectNamed = provider.provide();
    // Then
    assert_eq!(*svc.0, "postgres://localhost");
    assert_eq!(*svc.1, "secret-key");
}

#[injectable]
#[derive(Debug, PartialEq)]
struct ServiceWithDirectNamedFields {
    db_url: Named<DbUrl, String>,
    api_key: Named<ApiKey, String>,
}

#[test]
fn provide_direct_named_named_struct() {
    // Given
    #[provider]
    #[provide(Named<DbUrl, String>, Named::new("postgres://db".into()))]
    #[provide(Named<ApiKey, String>, Named::new("my-api-key".into()))]
    struct DirectFieldProvider;

    let provider = DirectFieldProvider;
    // When
    let svc: ServiceWithDirectNamedFields = provider.provide();
    // Then
    assert_eq!(*svc.db_url, "postgres://db");
    assert_eq!(*svc.api_key, "my-api-key");
}

#[injectable]
#[derive(Debug, PartialEq)]
struct ServiceWithNamedSugar {
    #[inject(named(DbUrl))]
    db_url: String,
    #[inject(named(ApiKey))]
    api_key: String,
}

#[test]
fn provide_named_sugar_on_named_struct() {
    // Given
    #[provider]
    #[provide(Named<DbUrl, String>, Named::new("postgres://sugar".into()))]
    #[provide(Named<ApiKey, String>, Named::new("sugar-key".into()))]
    struct SugarProvider;

    let provider = SugarProvider;
    // When
    let svc: ServiceWithNamedSugar = provider.provide();
    // Then — fields are plain Strings, no Named wrapper
    assert_eq!(svc.db_url, "postgres://sugar");
    assert_eq!(svc.api_key, "sugar-key");
}

#[injectable]
#[derive(Debug, PartialEq)]
struct ServiceWithNamedSugarTuple(
    #[inject(named(DbUrl))] String,
    #[inject(named(ApiKey))] String,
);

#[test]
fn provide_named_sugar_on_tuple_struct() {
    // Given
    #[provider]
    #[provide(Named<DbUrl, String>, Named::new("pg://tuple".into()))]
    #[provide(Named<ApiKey, String>, Named::new("tuple-key".into()))]
    struct TupleSugarProvider;

    let provider = TupleSugarProvider;
    // When
    let svc: ServiceWithNamedSugarTuple = provider.provide();
    // Then
    assert_eq!(svc.0, "pg://tuple");
    assert_eq!(svc.1, "tuple-key");
}

#[injectable]
#[derive(Debug, PartialEq)]
struct DualPorts {
    #[inject(named(Primary))]
    primary_port: u16,
    #[inject(named(Secondary))]
    secondary_port: u16,
}

#[test]
fn provide_same_type_different_names_gives_different_values() {
    // Given
    #[provider]
    #[provide(Named<Primary, u16>, Named::new(8080))]
    #[provide(Named<Secondary, u16>, Named::new(9090))]
    struct PortProvider;

    let provider = PortProvider;
    // When
    let ports: DualPorts = provider.provide();
    // Then
    assert_eq!(ports.primary_port, 8080);
    assert_eq!(ports.secondary_port, 9090);
}

#[test]
fn provide_named_from_stateful_provider_fields() {
    // Given
    #[provider]
    #[provide(Named<Primary, i32>, Named::new(self.primary))]
    #[provide(Named<Secondary, i32>, Named::new(self.secondary))]
    struct StatefulProvider {
        primary: i32,
        secondary: i32,
    }

    #[injectable]
    #[derive(Debug, PartialEq)]
    struct TwoInts {
        #[inject(named(Primary))]
        a: i32,
        #[inject(named(Secondary))]
        b: i32,
    }

    let provider = StatefulProvider {
        primary: 111,
        secondary: 222,
    };
    // When
    let result: TwoInts = provider.provide();
    // Then
    assert_eq!(result.a, 111);
    assert_eq!(result.b, 222);
}

#[injectable]
#[derive(Debug, PartialEq)]
struct RegularDep(#[inject(42)] i32);

#[injectable]
#[derive(Debug, PartialEq)]
struct MixedService {
    #[inject(named(DbUrl))]
    db_url: String,
    regular: RegularDep,
}

#[test]
fn provide_mixed_named_and_regular_injection() {
    // Given
    #[provider]
    #[provide(Named<DbUrl, String>, Named::new("postgres://mixed".into()))]
    struct MixedProvider;

    let provider = MixedProvider;
    // When
    let svc: MixedService = provider.provide();
    // Then
    assert_eq!(svc.db_url, "postgres://mixed");
    assert_eq!(svc.regular, RegularDep(42));
}

#[test]
fn key_type_is_zero_sized() {
    assert_eq!(core::mem::size_of::<Key<0>>(), 0);
    assert_eq!(core::mem::size_of::<Key<12345>>(), 0);
    // Named<Key<H>, T> is same size as T
    assert_eq!(
        core::mem::size_of::<Named<Key<0>, String>>(),
        core::mem::size_of::<String>()
    );
}

#[test]
fn str_key_hash_is_deterministic() {
    assert_eq!(str_key_hash("db_url"), str_key_hash("db_url"));
    assert_ne!(str_key_hash("db_url"), str_key_hash("api_key"));
}

#[test]
fn key_macro_matches_str_key_hash() {
    // key!("x") should produce the same type as Key<{str_key_hash("x")}>
    fn assert_same_type<T>(_a: T, _b: T) {}
    let a: Named<key!("db_url"), i32> = Named::new(1);
    let b: Named<Key<{ str_key_hash("db_url") }>, i32> = Named::new(2);
    assert_same_type(a, b);
}

#[injectable]
#[derive(Debug, PartialEq)]
struct StrKeyService {
    #[inject(named("db_url"))]
    db_url: String,
    #[inject(named("api_key"))]
    api_key: String,
}

#[test]
fn provide_named_str_on_named_struct() {
    // Given
    #[provider]
    #[provide(Named<key!("db_url"), String>, Named::new("pg://str-key".into()))]
    #[provide(Named<key!("api_key"), String>, Named::new("str-secret".into()))]
    struct StrKeyProvider;

    let provider = StrKeyProvider;
    // When
    let svc: StrKeyService = provider.provide();
    // Then
    assert_eq!(svc.db_url, "pg://str-key");
    assert_eq!(svc.api_key, "str-secret");
}

#[injectable]
#[derive(Debug, PartialEq)]
struct StrKeyTupleService(
    #[inject(named("primary_port"))] u16,
    #[inject(named("secondary_port"))] u16,
);

#[test]
fn provide_named_str_on_tuple_struct() {
    // Given
    #[provider]
    #[provide(Named<key!("primary_port"), u16>, Named::new(8080))]
    #[provide(Named<key!("secondary_port"), u16>, Named::new(9090))]
    struct StrKeyPortProvider;

    let provider = StrKeyPortProvider;
    // When
    let svc: StrKeyTupleService = provider.provide();
    // Then
    assert_eq!(svc.0, 8080);
    assert_eq!(svc.1, 9090);
}

#[test]
fn provide_named_str_from_stateful_provider() {
    // Given
    #[provider]
    #[provide(Named<key!("host"), String>, Named::new(self.host.clone()))]
    #[provide(Named<key!("port"), u16>, Named::new(self.port))]
    struct ConfigProvider {
        host: String,
        port: u16,
    }

    #[injectable]
    #[derive(Debug, PartialEq)]
    struct AppConfig {
        #[inject(named("host"))]
        host: String,
        #[inject(named("port"))]
        port: u16,
    }

    let provider = ConfigProvider {
        host: "localhost".into(),
        port: 3000,
    };
    // When
    let config: AppConfig = provider.provide();
    // Then
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 3000);
}

#[injectable]
#[derive(Debug, PartialEq)]
struct FullyMixedService {
    #[inject(named("connection_string"))]
    conn: String,
    #[inject(named(Primary))]
    primary_id: u32,
    regular: RegularDep,
}

#[test]
fn provide_mixed_str_key_type_tag_and_regular() {
    // Given
    #[provider]
    #[provide(Named<key!("connection_string"), String>, Named::new("pg://mixed-all".into()))]
    #[provide(Named<Primary, u32>, Named::new(999))]
    struct FullyMixedProvider;

    let provider = FullyMixedProvider;
    // When
    let svc: FullyMixedService = provider.provide();
    // Then
    assert_eq!(svc.conn, "pg://mixed-all");
    assert_eq!(svc.primary_id, 999);
    assert_eq!(svc.regular, RegularDep(42));
}

#[injectable]
#[derive(Debug, PartialEq)]
struct SameKeyDifferentTypes {
    #[inject(named("value"))]
    as_string: String,
    #[inject(named("value"))]
    as_int: i32,
    #[inject(named("value"))]
    as_float: f64,
}

#[test]
fn provide_same_str_key_different_types_resolves_independently() {
    // Given — same key "value" but three different types
    #[provider]
    #[provide(Named<key!("value"), String>, Named::new("hello".into()))]
    #[provide(Named<key!("value"), i32>, Named::new(42))]
    #[provide(Named<key!("value"), f64>, Named::new(3.14))]
    struct SameKeyProvider;

    let provider = SameKeyProvider;
    // When
    let svc: SameKeyDifferentTypes = provider.provide();
    // Then — each field gets the value for its own type
    assert_eq!(svc.as_string, "hello");
    assert_eq!(svc.as_int, 42);
    assert_eq!((svc.as_float - 3.14).abs() < f64::EPSILON, true);
}

#[injectable]
#[derive(Debug, PartialEq)]
struct SameKeyDifferentTypesTuple(
    #[inject(named("data"))] String,
    #[inject(named("data"))] u64,
    #[inject(named("data"))] bool,
);

#[test]
fn provide_same_str_key_different_types_tuple_resolves_independently() {
    // Given
    #[provider]
    #[provide(Named<key!("data"), String>, Named::new("world".into()))]
    #[provide(Named<key!("data"), u64>, Named::new(999))]
    #[provide(Named<key!("data"), bool>, Named::new(true))]
    struct SameKeyTupleProvider;

    let provider = SameKeyTupleProvider;
    // When
    let svc: SameKeyDifferentTypesTuple = provider.provide();
    // Then
    assert_eq!(svc.0, "world");
    assert_eq!(svc.1, 999);
    assert_eq!(svc.2, true);
}

// Same type-tag, different types — verify same behavior with type tags
#[injectable]
#[derive(Debug, PartialEq)]
struct SameTagDifferentTypes {
    #[inject(named(Primary))]
    as_string: String,
    #[inject(named(Primary))]
    as_int: i32,
}

#[test]
fn provide_same_type_tag_different_types_resolves_independently() {
    // Given
    #[provider]
    #[provide(Named<Primary, String>, Named::new("tagged".into()))]
    #[provide(Named<Primary, i32>, Named::new(77))]
    struct SameTagProvider;

    let provider = SameTagProvider;
    // When
    let svc: SameTagDifferentTypes = provider.provide();
    // Then
    assert_eq!(svc.as_string, "tagged");
    assert_eq!(svc.as_int, 77);
}
