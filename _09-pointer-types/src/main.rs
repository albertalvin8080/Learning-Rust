#![allow(warnings)]
fn main() {
    let mut b1: Box<i32> = Box::new(32);
    *b1 += 1; // OK

    let b_ref: &Box<i32> = &b1;
    // *b1 += 1; // ERR: assigned to here but it was already borrowed (to b_ref)

    println!("{b1}");
    println!("{b_ref}");
}
