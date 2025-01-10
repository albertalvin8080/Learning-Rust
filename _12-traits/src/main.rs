#![allow(warnings)]

use std::process::Output;

#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // Associated Function
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point<T>
where
    T: PartialEq + PartialOrd,
{
    fn compare_x_y(&self) -> &str {
        if self.x >= self.y {
            "x"
        } else {
            "y"
        }
    }
}

fn main() {
    let p = Point::new(3, 6);
    let p2 = Point::new(3, 3);
    let p3 = Point::new(5, 3);
    println!("{}", p.compare_x_y());
    println!("{}", p2.compare_x_y());
    println!("{}", p3.compare_x_y());
}
