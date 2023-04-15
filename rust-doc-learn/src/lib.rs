use std::fmt::{self, Debug};

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

pub trait Summary {
    //fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /*fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }*/
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /*fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }*/
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.summarize(), self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_same_trait_type(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "** Same trait type {} {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_trait_bound_syntax<T: Summary>(item1: &T, item2: &T) {
    println!(
        "** Trait bound syntax {} {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_trait_bound_with_plus_sign(item: &(impl Summary + fmt::Display + Debug)) {
    println!("** Trait bound syntax {:#?}", item);
    //format!("@{:?}", item)
}

pub fn _notify3<T: Summary + fmt::Display>(_item: &T) {}

fn _some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: fmt::Display + Clone,
    U: Clone + fmt::Debug,
{
    50
}

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: fmt::Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


