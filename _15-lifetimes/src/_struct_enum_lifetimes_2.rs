#[derive(Debug)]
struct NoCopyType;

#[derive(Debug)]
struct Borrower<'a> {
    x: &'a i32,
    y: &'a NoCopyType,
}

fn f<'a>(borrower: &'a Borrower) -> &'a NoCopyType {
    &borrower.y
}

pub fn run() {
    let borrower: Borrower<'_>;
    let nct = NoCopyType;
    
    // borrower = Borrower { x: &32, y: &nct };
    borrower = Borrower { x: &32, y: &NoCopyType };
    f(&borrower);

    println!("OK");
}
