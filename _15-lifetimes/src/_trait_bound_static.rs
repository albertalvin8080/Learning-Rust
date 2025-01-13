pub fn run() {
    let n: i32 = 34;

    print_it::<i32>(n); // OK
    // print_it::<i32>(&n);   // ERR

    print_it_v2(n); // OK
    // print_it_v2(&n);   // ERR
    
    print_it_v3::<i32>(&n); // OK

    let temp: i32 = 43;
    let m: MyType<'_> = MyType(&temp);
    // print_it(m); // ERR: `temp` is non 'static

    let m: MyType<'_> = MyType(&43);
    print_it(m); // OK
}

#[derive(Debug)]
struct MyType<'a>(&'a i32);

use std::fmt::Debug;

// Can't contain any non-static references. Meaning it can only have owned types and/or 'static references. 
fn print_it<T: Debug + 'static>(param: T) {
    println!("{:?}", param);
}

fn print_it_v2(param: impl Debug + 'static) {
    println!("{:?}", param);
}

fn print_it_v3<T: Debug + 'static>(param: &T) {
    println!("{:?}", param);
}