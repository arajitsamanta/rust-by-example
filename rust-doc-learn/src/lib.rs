mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Restaurant - In waitlist")
        }

        pub fn seat_at_table() {
            println!("Restaurant - Got a table")
        }
    }

    pub mod serving {
        pub fn _take_order() {}

        pub fn _serve_order() {}

        pub fn _take_payment() {}
    }
}

fn _deliver_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();
        super::_deliver_order();
    }

    fn _cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::seat_at_table();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
