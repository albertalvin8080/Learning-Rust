#![allow(warnings)]
fn main() {
    let r = divideOption(32., 3.);
    match r {
        Some(x) => println!("{x}"),
        None => println!("Cannot divide by zero!")
    }

    let r = divideResult(32., 0.);
    match r {
        Ok(x) => println!("{x}"),
        Err(e) => println!("{e}")
    }
}

// Option<T>
fn divideOption(a: f32, b:f32) -> Option<f32> {
    if b == 0.0 {
        None
    }
    else {
        Some(a / b) // Some(T) exists within Option<T>
    }
}

// Result<T, E>
fn divideResult(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string()) // Constructing an enum variant of Result
    } else {
        Ok(a / b) // Constructing an enum variant of Result
    }
}
