#![allow(warnings)]
fn main() {
    // label_loop();
    evaluate_loop();
}

fn evaluate_loop() {
    let mut n = 0;

    let x = loop {
        n += 1;
        if n >= 10 {
            break n * 2; // evaluates to n * 2
        }
    };
    
    println!("{x}");
}

fn label_loop() {
    let mut outer_n = 0;
    'outer: loop {
        let mut inter_n = 0;

        loop {
            println!("outer: {outer_n}");
            println!("inter: {inter_n}");

            if inter_n >= 2 {
                break;
            }

            if outer_n >= 2 {
                break 'outer;
            }

            inter_n += 1;
        }

        outer_n += 1;
    }
}
