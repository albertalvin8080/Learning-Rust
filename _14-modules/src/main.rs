#![allow(warnings)]

mod back_of_house;
mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    // We cannot use `crate::`` inside main.rs. Use the name of the package instead.
    assert_eq!(_14_modules::eat_at_restaurant(), "yummy yummy!");
}
