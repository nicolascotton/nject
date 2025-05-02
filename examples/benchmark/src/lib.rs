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

use nject::{inject, injectable, module};

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

pub trait MultiTrait {}

impl MultiTrait for Dep1 {}
impl MultiTrait for Dep2 {}
impl MultiTrait for Dep3 {}
impl MultiTrait for Dep4 {}
impl MultiTrait for Dep5 {}
impl MultiTrait for Dep6 {}
impl MultiTrait for Dep7 {}
impl MultiTrait for Dep8 {}
impl MultiTrait for Dep9 {}
impl MultiTrait for Dep10 {}
struct MultiDep(i32);

#[injectable]
#[module(crate::Self)]
#[export(&'prov dyn crate::MultiTrait, &self.dep1)]
#[export(&'prov dyn crate::MultiTrait, &self.dep2)]
#[export(&'prov dyn crate::MultiTrait, &self.dep3)]
#[export(&'prov dyn crate::MultiTrait, &self.dep4)]
#[export(&'prov dyn crate::MultiTrait, &self.dep5)]
#[export(&'prov dyn crate::MultiTrait, &self.dep6)]
#[export(&'prov dyn crate::MultiTrait, &self.dep7)]
#[export(&'prov dyn crate::MultiTrait, &self.dep8)]
#[export(&'prov dyn crate::MultiTrait, &self.dep9)]
#[export(&'prov dyn crate::MultiTrait, &self.dep10)]
#[export(crate::MultiDep, MultiDep(1))]
#[export(crate::MultiDep, MultiDep(2))]
#[export(crate::MultiDep, MultiDep(3))]
#[export(crate::MultiDep, MultiDep(4))]
#[export(crate::MultiDep, MultiDep(5))]
#[export(crate::MultiDep, MultiDep(6))]
#[export(crate::MultiDep, MultiDep(7))]
#[export(crate::MultiDep, MultiDep(8))]
#[export(crate::MultiDep, MultiDep(9))]
#[export(crate::MultiDep, MultiDep(10))]
struct MultiExportModule {
    dep1: Dep1,
    dep2: Dep2,
    dep3: Dep3,
    dep4: Dep4,
    dep5: Dep5,
    dep6: Dep6,
    dep7: Dep7,
    dep8: Dep8,
    dep9: Dep9,
    dep10: Dep10,
}
