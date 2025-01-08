#![allow(warnings)]

// Update Syntax
fn main() {
    #[derive(Debug, Default)]
    struct User {
        name: String,
        email: String,
        active: bool,
        option_field: Option<u32>,
    }

    let mut u1 = User {
        name: String::from("Franz Bonaparta"),
        email: String::from("fb@mail.com"),
        ..Default::default() // Fill in fields with default values
    };

    println!("{:?}", u1);

    // struct update syntax
    let mut u2 = User {
        name: String::from("Helmuth Voss"),
        ..u1
    };

    println!("{:?}", u2);
    // println!("{:?}", u1); // String email was moved, so it cannot be borrowed here.
}
