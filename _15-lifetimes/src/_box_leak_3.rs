pub fn run() {
    let mut string = "First".to_owned();

    /* 
    WHY a, b, e AND f WORK FINE FOR &string?

    Short answer: 
    >> Because the references don't actually need to be 'static, only the concrete types 
    (that means the concrete types must not have references to non-static fiels inside themselves).

    Long answer:
    >> A type bound like T: Display + 'static means the type T must be valid for the 
    'static lifetime (e.g., the type doesn't have non-static references).
    >> However, references to values (&T) are not required to have a 'static lifetime 
    if the type itself satisfies 'static.
    >> Trait objects like &(dyn Display) or &(dyn Display + 'static) behave differently 
    from references to concrete types. The 'static in &(dyn Display + 'static) refers 
    to the trait object itself being valid for 'static.
    */

    string.push_str(string.to_uppercase().as_str());
    print_a(&string); // OK
    print_b(&string); // OK

    let leaked_string: &'static mut String = Box::leak(Box::new(string.clone()));
    
    // print_c(&string); // Compilation error
    print_c(leaked_string); // OK

    // print_d(&string); // Compilation error
    print_d(leaked_string); // OK

    print_e(&string); // OK
    print_f(&string); // OK

    // print_g(&string); // Compilation error
    print_g(leaked_string); // OK
}

use std::fmt::Display;

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

fn print_c(t: &'static dyn Display) {
    println!("{}", t)
}

fn print_d(t: &'static impl Display) {
    println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

fn print_g(t: &'static String) {
    println!("{}", t);
}
