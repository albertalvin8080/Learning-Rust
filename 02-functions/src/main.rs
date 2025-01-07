fn main() 
{
    let x = add(34, 2);
    println!("{}", x);

    // This block is evaluated to the last line (it must have no semicolon)
    let y = {
        let a = 30;
        let b = 10;
        a * b
    };
    println!("{}", y);
}

// Hoisting
fn add(a: i32, b: i32) -> i32
{
    a * b
}
