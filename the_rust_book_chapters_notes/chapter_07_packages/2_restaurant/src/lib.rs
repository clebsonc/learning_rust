// a module is defined by using the keyword mod.
// a module may have inner modules.
// Modules can also hold definitions for other items, such as structs, enums, constants, traits,
// and functions.

mod front_of_house {
    pub mod hosting {

        // Each module is basically a container, and everything in this container is private by
        // default. If you ought to make inner contents be accessed from outside, it is necessary
        // to use the keyword `pub` to allow content ouside of this container to acess it.
        pub fn add_to_waitlist() {}    // using the keyworkd pub here makes it public.

        #[allow(dead_code)]
        fn seat_at_table() {}   // since this function does not have the keyword pub, it is
                                    // private.
    }

    mod serving {
        #[allow(dead_code)]
        fn take_order() {}

        #[allow(dead_code)]
        fn serve_order() {}

        #[allow(dead_code)]
        fn take_payment() {}
    }
}



// By default rust considers a tree of modules, having a parent module named `crate`. For example,
// the above module schema will look like:

// crate
//      front_of_house
//          hosting
//              add_to_waitlist
//              seat_at_table
//          serving
//              take_order
//              serve_order
//              take_payment


// The `crate` module is named after the files `src/main.rs` or `src/lib.rs`.
// For example this function bellow is in the crate module:


#[allow(dead_code)]
fn deliver_order() {}


pub fn eat_at_restaurant() {
    // Using absolute path to access a module.
    crate::front_of_house::hosting::add_to_waitlist();

    // Or, since the module front of house is in the same file of eat_at_restaurant, we can use a
    // relative path.
    front_of_house::hosting::add_to_waitlist();



}


// It is possible to construct relative paths that begins in the parent module. For example, in
// this module, the parent is the `crate` module. To load something from this module from a
// children module with relative paths it is possible to use the keyword `super`.

mod back_of_house {
    #[allow(dead_code)]
    fn cook_order () {}


    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();  // function in the module `back_of_house`
        super::eat_at_restaurant();  // function in the parent module, which in this case is the
                                     // crate module.
    }



    // Structs can also be made public, but their fields remaing private.
    // To make the fields public, the keyword `pub` should be used as well. Ex:
    pub struct Breakfast {
        pub toast: String,   // public string `toast`

        #[allow(dead_code)]
        seasonal_fruit: String,  // private string `seasonal_fruit`
    }

    impl Breakfast {
        #[allow(dead_code)]

        // Function that returns a Breakfast.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),

                // we can access the field `seasonal_field` here because it is being accessed inside the
                // function summer, that belongs to the struct Breakfast.
                seasonal_fruit: String::from("peaches"),  
            }
        }
    }

    // Enums can also be made public, and different from structs, once an enum is made public its
    // variants are also defined as public. For example, both variants Soup and Salad are made
    // public by default because the keyword `pub` is used before the `enum` keyword.
    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


pub fn eat_at_restaurant_2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Example of instantianting the enum example in back_of_house:
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Soup;
    println!("order 1: {:#?}", order1);
    println!("order 2: {:#?}", order2);

}

