// lib.rs
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
  mod serving {
    fn take_order() {}
  }
}

mod back_of_house {
    fn add_vip() {
        super::front_of_house::hosting::add_to_waitlist();
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
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


pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // 절대 경로
    front_of_house::hosting::add_to_waitlist(); //상대 경로
  
    // crate::front_of_house::serving::take_order(); // error
    // front_of_house::serving::take_order(); // error
    
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}