#![allow(warnings)]

// Partial referencing
fn main() {
    #[derive(Debug)]
    struct Author {
        name: String,
        age: Box<u8>,
    }

    let a1 = Author {
        name: String::from("Helmuth Voss"),
        age: Box::new(39)
    };

    let Author { name, ref age } = a1;

    // println!("{a1:?}"); // ERR
    println!("{:?}", a1.age);
    println!("{name}, {age}");
}
