pub fn run() {
    println!("{}", Point { x: 43.33, y: 743.2 });
    println!("{:?}", Point { x: 43.33, y: 743.2 });
}

use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display: x: {:.2}, y: {:.2}", self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug: x: {:.0}, y: {:.0}", self.x, self.y)
    }
}
