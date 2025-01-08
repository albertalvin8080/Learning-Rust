#![allow(warnings)]

use std::any::type_name;
use std::mem::size_of_val;
use std::mem::size_of;
fn main() {
    let t: &str = type_name::<u8>();
    let s: usize = size_of_val(t);
    println!("{t}, {s}");

    let s1: usize = size_of_val(&1);
    println!("{s1}");

    let s2: usize = size_of::<u128>();
    println!("{s2}");
}


