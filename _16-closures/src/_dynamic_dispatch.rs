pub fn run() {
    {
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }
    {
        let fun = create_fn();
        println!("{}", fun(2));
        // println!("{}", fun(2)); // ERR
    }
    {
        let fun: Box<dyn FnOnce(i32) -> i32> = fatcory(10);
        println!("{}", fun(23));
        let fun: Box<dyn FnOnce(i32) -> i32> = fatcory(4);
        println!("{}", fun(23));
    }
}

// Dynamic dispatch used because the closures returned have different memory adresses.
fn fatcory(x: i32) -> Box<dyn FnOnce(i32) -> i32> {
    let num: i32 = 5;

    if x > 5 {
        Box::new(move |x| x - 5 + num)
    } 
    else {
        Box::new(move |x| x + num)
    }
}

fn create_fn() -> impl FnOnce(i32) -> i32 {
    let num: i32 = 5;
    // num MUST be taken by value.
    move |x| x + num
}

fn call_me<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn function() {
    println!("I'm a function!");
}
