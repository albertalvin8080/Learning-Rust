use std::{
    sync::{Arc, Mutex, Weak},
    thread,
};

pub fn run() {
    let mut franz = Arc::new(Owner {
        name: "Franz Bonaparta".to_string(),
        tools: Mutex::new(vec![]),
    });
    let tool1 = Arc::new(Tool {
        name: "tool1".to_string(),
        owner: franz.clone(),
    });
    let tool2 = Arc::new(Tool {
        name: "tool2".to_string(),
        owner: Arc::clone(&franz),
    });

    {
        // Unlocks at end of scope
        franz.tools.lock().unwrap().push(Arc::downgrade(&tool1));
        franz.tools.lock().unwrap().push(Arc::downgrade(&tool2));
    }

    for _ in (0..5) {
        let tool1: Weak<Tool> = Arc::downgrade(&tool1);
        let tool2: Weak<Tool> = Arc::downgrade(&tool2);
        let franz: Arc<Owner> = Arc::clone(&franz);
        let t = thread::spawn(move || {
            let mut guard = franz.tools.lock().unwrap();

            let tool: Arc<Tool> = tool1.upgrade().unwrap();
            println!("{:?}:{:?}", tool.name, tool.owner.name);
            guard.push(Arc::downgrade(&tool));

            let tool: Arc<Tool> = tool2.upgrade().unwrap();
            println!("{:?}:{:?}", tool.name, tool.owner.name);
            guard.push(Arc::downgrade(&tool));
        });
        t.join();
    }

    println!("{:#?}", franz);
}

#[derive(Debug)]
struct Owner {
    name: String,
    tools: Mutex<Vec<Weak<Tool>>>,
}

#[derive(Debug)]
struct Tool {
    name: String,
    owner: Arc<Owner>,
}
