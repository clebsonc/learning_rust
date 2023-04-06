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

        fn seat_at_table() {}   // since this function does not have the keyword pub, it is
                                    // private.
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

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


pub fn eat_at_restaurant() {
    // Using absolute path to access a module.
    crate::front_of_house::hosting::add_to_waitlist();

    // Or, since the module front of house is in the same file of eat_at_restaurant, we can use a
    // relative path.
    front_of_house::hosting::add_to_waitlist();
}
