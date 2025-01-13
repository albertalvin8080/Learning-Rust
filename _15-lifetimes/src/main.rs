#![allow(warnings)]

mod _struct_enum_lifetimes_1;
mod _struct_enum_lifetimes_2;
mod _func_lifetimes;
mod _coercing_static;
mod _struct_lifetime_impl;
mod _trait_bound_static;
mod _box_leak_1;
mod _box_leak_2;
mod _box_leak_3;

fn main() {
    // _struct_enum_lifetimes_1::run();
    // _struct_enum_lifetimes_1::run2();
    // _struct_enum_lifetimes_2::run();
    // _func_lifetimes::run();
    // _coercing_static::run();
    // _struct_lifetime_impl::run();
    // _trait_bound_static::run();
    // _box_leak_1::run();
    // _box_leak_2::run();
    _box_leak_3::run();
}
