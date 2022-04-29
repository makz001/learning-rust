mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        // using a relative path starting with super
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
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

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    // The next line won't compile if we uncomment it; we're not allowed
    // to see of modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant3() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
