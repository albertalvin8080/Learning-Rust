#![allow(warnings)]

mod object_traits;
mod operators_traits;
mod where_for_trait_bounds;
mod object_safe_traits;

fn main () {
    object_traits::run();
    operators_traits::run();
    where_for_trait_bounds::run();
    object_safe_traits::run();
}