#[allow(overflowing_literals, warnings)]
fn main() {
    println!("{}", 1000 as u8);
    println!("{}", 1000 % 256);
    println!("{}", 1000 as u16);
    println!("{}", 'a' as u8);
    println!("{}", 99.66 as u8 as char);
    println!("{}", 97.66f64 as u8 as char);

    unsafe {
        let n = 300.1_f32.to_int_unchecked::<u8>();
        println!("{n}");
        let n = 1_122.4322_f64.to_int_unchecked::<u8>();
        println!("{n}");
        let n = (-1_122.4322_f64).to_int_unchecked::<u8>();
        println!("{n}");
    }
}
