use std::ops::*;
#[derive(Debug)]
struct Awareness;
#[derive(Debug)]
struct Human;
#[derive(Debug)]
struct SelfAwareness;

impl Add<Awareness> for Human {
    type Output = SelfAwareness;

    fn add(self, rhs: Awareness) -> Self::Output {
        // SelfAwareness {}
        Self::Output {}
    }
}

impl Sub<Awareness> for SelfAwareness {
    type Output = Human;

    fn sub(self, rhs: Awareness) -> Self::Output {
        // Human {}
        Self::Output {}
    }
}

pub fn run() {
    let h = Human;
    let a = Awareness;
    println!("{:?}", h);
    println!("{:?}", a);

    let sa = Human + Awareness;
    println!("{:?}", sa);

    let h2 = sa - Awareness;
    println!("{:?}", h2);
}
