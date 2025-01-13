pub fn run() {
    let b: Box<String> = Box::new(String::from("Franz"));

    // Box::leak(Box<T>) is mainly useful for data that lives 
    // for the remainder of the program's life. Dropping 
    // the returned reference will cause a memory leak.
    let l: &'static mut String = Box::leak(b);
    println!("{}", l);
    l.push_str(" Bonaparta");
    println!("{}", l);
}

