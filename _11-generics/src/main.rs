#![allow(warnings)]

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: Clone,
{
    pub fn mixup<V, W: Clone>(&self, other: &Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x.clone(),
            y: other.y.clone(),
        }
    }
}

fn main() {
    let p1 = Point{
        x: '愛',
        y: "Franz",
    };

    let p2= Point{
        x: 3 as u32,
        y: 74 as f64,
    };

    let p3 = p1.mixup(&p2);

    println!("{:?}", p3);
}
