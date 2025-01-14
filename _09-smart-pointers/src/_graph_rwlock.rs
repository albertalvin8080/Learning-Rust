use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
    thread,
};

pub fn run() {
    let mut a: Arc<Node> = Arc::new(Node {
        value: RwLock::new(String::from("Franz Bonaparta")),
        adjacent: vec![],
    });

    let mut b: Arc<Node> = Arc::new(Node {
        value: RwLock::new(String::from("Helmuth Voss")),
        adjacent: vec![Arc::clone(&a)],
    });

    let mut c: Arc<Node> = Arc::new(Node {
        value: RwLock::new(String::from("Jakub Farobek")),
        adjacent: vec![Arc::clone(&a), Arc::clone(&b)],
    });

    // This creates another pointer to the same allocation, increasing the strong reference count.
    let t1_c: Arc<Node> = c.clone();
    let t1 = thread::spawn(move || add_mark(&t1_c));
    
    let t2_b: Arc<Node> = b.clone();
    let t2 = thread::spawn(move || add_mark(&t2_b));

    t1.join();
    t2.join();

    println!("{:#?}", c);
}

#[derive(Debug)]
struct Node {
    value: RwLock<String>,
    adjacent: Vec<Arc<Node>>,
}

fn add_mark(node: &Node) {
    // Inside a block because we want the lock to be released as soon as possible.
    {
        let b = node.value.write();
        if let Ok(mut v) = b {
            v.push_str("!");
        }
    }
    // n: &Arc<Node>
    for n in node.adjacent.iter() {
        add_mark(n);
    }
}
