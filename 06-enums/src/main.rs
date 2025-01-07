#![allow(warnings)]
fn main() {
    #[derive(Debug)]
    enum IpAddrType {
        V4(u8, u8, u8, u8), // They may or may not have values.
        V6(String),
        VTest,
    }

    let ipV4 = IpAddrType::V4(127, 0, 0, 1);
    let ipV6 = IpAddrType::V6(String::from("::1"));
    let ipVTest = IpAddrType::VTest;

    println!("{ipV4:?}");
    println!("{ipV6:?}");
    println!("{ipVTest:?}");
}
