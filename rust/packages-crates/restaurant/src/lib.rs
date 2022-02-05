
mod front_of_house; // Load from other file

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // Not public
    }

    pub enum Appetizer { // public by default
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


    fn fix_incorrect_order() {
        cook_order();
        // Super retruns goes up the hierarchy (e.g. doing this in fs ..)
        // In this case it returns to the root
        //super::front_of_house::serving::serve_order();
    }

    fn cook_order() {

    }
}


pub fn eat_at_restaurant() {
    // Pathing in rust is extremely similar to pathing through file systems
        
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // or
    // use self.::front_of_house::hosting;
    // then 
    // hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye"); // Order rye
    meal.toast = String::from("Wheat"); // Change mind
    println!("I'd like {} toast please", meal.toast);


}


