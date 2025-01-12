pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn sit_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
        pub fn complain() {}
    }
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        // using super (goes up one level)
        super::front_of_house::serving::serve_order();
        // using absolute path
        crate::front_of_house::serving::serve_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // using absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // using relative path
    front_of_house::hosting::add_to_waitlist();
}
