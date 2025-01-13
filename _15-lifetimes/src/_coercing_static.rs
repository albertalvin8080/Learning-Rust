pub fn run() {
    println!("{}", coerce_lifetime(&3));
}

static NUM_1: u8 = 43;
const NUM_2: u8 = 89;

fn coerce_lifetime<'a>(_:&'a u8) -> &'a u8 {
    &NUM_1
    // &NUM_2 // Also works.
}