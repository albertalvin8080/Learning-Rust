pub fn run() {
    unsafe {
        config = init();

        println!("{:?}", config);
    }
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

/* 
NOTE: Accessing `static mut` variables is inherently unsafe because it can lead to 
undefined behavior if not synchronized properly in multithreaded environments.

Problem: you want to assign a new mutable Config reference (&mut Config) to this variable, but
this variable is static. It means it will live for the entire program. With that in mind, you 
must create a &'static mut Config reference because it will ALSO live for the entire program.
*/
static mut config: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    // ERR: Where is the 'static lifetime of &mut Config?
    // Some(&mut Config {
    // a: "A".to_string(),
    // b: "B".to_string(),
    // })

    /* 
    1. Allocate Config{} into heap memory.
    2. Leak it (make it acessible for the entire rest of the program, and thus 'static).
    3. Wrap it inside Option<>.
    */
    let b: Box<Config> = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    let b: &'static mut Config = Box::leak(b);

    Some(b)
}
