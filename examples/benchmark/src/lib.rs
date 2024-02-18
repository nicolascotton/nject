#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(type_alias_impl_trait))]
#![allow(dead_code)]

#[cfg(test)]
extern crate test;
#[cfg(test)]
pub mod baseline;
#[cfg(test)]
pub mod module;
#[cfg(test)]
pub mod provide;
#[cfg(test)]
pub mod scope;

use nject::{inject, injectable};

pub const ITERATION_COUNT: i32 = 10000;

#[inject(Self(123))]
pub struct Dep1(pub i32);

#[injectable]
pub struct Dep2(Dep1);

#[injectable]
pub struct Dep3(Dep1, Dep2);

#[injectable]
pub struct Dep4(Dep1, Dep2, Dep3);

#[injectable]
pub struct Dep5(Dep1, Dep2, Dep3, Dep4);

#[injectable]
pub struct Dep6(Dep1, Dep2, Dep3, Dep4, Dep5);

#[injectable]
pub struct Dep7(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6);

#[injectable]
pub struct Dep8(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7);

#[injectable]
pub struct Dep9(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8);

#[injectable]
pub struct Dep10(Dep1, Dep2, Dep3, Dep4, Dep5, Dep6, Dep7, Dep8, Dep9);

pub trait DepTrait1 {}
pub trait DepTrait2 {}
pub trait DepTrait3 {}
pub trait DepTrait4 {}
pub trait DepTrait5 {}
pub trait DepTrait6 {}
pub trait DepTrait7 {}
pub trait DepTrait8 {}
pub trait DepTrait9 {}
pub trait DepTrait10 {}

impl DepTrait1 for Dep1 {}
impl DepTrait2 for Dep2 {}
impl DepTrait3 for Dep3 {}
impl DepTrait4 for Dep4 {}
impl DepTrait5 for Dep5 {}
impl DepTrait6 for Dep6 {}
impl DepTrait7 for Dep7 {}
impl DepTrait8 for Dep8 {}
impl DepTrait9 for Dep9 {}
impl DepTrait10 for Dep10 {}
