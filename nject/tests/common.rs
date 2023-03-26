use nject::{injectable, provide, provider};

#[injectable]
#[derive(Debug, PartialEq)]
pub struct StructWithoutDeps;

#[injectable]
#[derive(Debug, PartialEq)]
pub struct StructWithNamedDeps {
    pub dep: StructWithoutDeps,
}

#[injectable]
#[derive(Debug, PartialEq)]
pub struct StructWithUnnamedDeps(pub StructWithoutDeps);

#[injectable]
#[derive(Debug, PartialEq)]
pub struct StructWithNamedGenericDeps<T> {
    pub dep: T,
}

#[injectable]
#[derive(Debug, PartialEq)]
pub struct StructWithUnnamedGenericDeps<T>(pub T);

#[injectable]
#[derive(Debug, PartialEq)]
pub struct StructWithNamedDepAndLifetime<'a> {
    pub dep: &'a StructWithoutDeps,
}

#[injectable]
#[derive(Debug, PartialEq)]
pub struct StructWithUnnamedDepAndLifetime<'a>(pub &'a StructWithoutDeps);

#[provider]
#[provide(i32, 11)]
pub struct CommonProvider;
