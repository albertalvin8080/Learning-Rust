#![allow(warnings)]

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy> Point<T> {
    pub fn new(x:T, y:T) -> Self {
        Self {
            x, y,
        }
    }

    pub fn add(&self) -> T {
        self.x + self.y
    }

    pub fn sub(&self) -> T {
        self.x - self.y
    }
}

// Only for <f64> types
impl Point<f64> {
    pub fn distance_from_origin(self: &Self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn angle_with_x(&self) -> f64 {
        self.y.atan2(self.x)
    }
}

fn main() {
    let pf64 = Point::new(3.0, 4.0);
    println!("{}", pf64.distance_from_origin());
    println!("{}", pf64.angle_with_x());
    println!("{}", pf64.add());
    println!("{}", pf64.sub());
    
    let pu8 = Point::new(32 as u8, 12 as u8);
    println!("{}", pu8.add());
    println!("{}", pu8.sub());
}
