fn main() {
    println!("{}", PI);
    println!("{}", E);

    let mut x: i32 = 34;
    let r_x: &mut i32 = &mut x;

    *r_x += 2;

    // println!("{}", x); // cannot borrow `x` as immutable because it is also borrowed as mutable (print below)
    println!("{}", r_x);
}

const PI: f32 = 3.1415;
const E: f32 = 2.718;
