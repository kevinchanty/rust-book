mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist...")
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
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
}

fn deliver_order() {}

mod customer {
    pub fn eat_at_restaurant() {
        use crate::front_of_house::hosting;

        hosting::add_to_waitlist();

        let mut meal = crate::back_of_house::Breakfast::summer("Rye");

        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast)
    }
}

// Next: Separating Modules into Different Fiels
