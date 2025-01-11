#[derive(Debug)]
struct Number(i32);

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

pub fn run() {
    let n1 = Number::from(44_i32);
    let n2: Number = 44_i32.into();

    println!("{:?}", n1);
    println!("{:?}", n2);
}
