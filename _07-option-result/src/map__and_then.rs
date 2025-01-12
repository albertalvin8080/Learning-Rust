use std::num::ParseIntError;

pub fn run() {
    if let Ok(n) = parse_and_sum("10", "2") {
        println!("{}", n);
    }

    match parse_and_sum_chaining("12", "34") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }

    match parse_and_sum_chaining("t", "112") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn parse_and_sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // map()      -> The closure must return V value.
    // and_then() -> The closure must return Result<V, ...> value.
    a.parse::<i32>()
        .and_then(|n1: i32| b.parse::<i32>().map(|n2: i32| n1 + n2))
}

fn parse_and_sum_chaining(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let n1 = a.parse::<i32>()?;
    let n2 = b.parse::<i32>()?;
    Ok(n1 + n2)
}
