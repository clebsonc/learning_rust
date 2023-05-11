//Closures can capture values from their environment in three ways, which directly map to the three
//ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking
//ownership.
use std::thread;

#[allow(dead_code)]
fn borrows() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list); // this only borrows the list

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

#[allow(dead_code)]
fn captures_mutable_reference() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[allow(dead_code)]
fn force_closure_take_ownership() {
    // use the `move` clause to force a closure to take ownership of the value.
    // Thread example:
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || {
        list.push(4);
        println!("From thread: {:?}", list)
    })
    .join()
    .unwrap();
    println!("After calling closure");
}

fn main() {
    // borrows();
    // println!("-----------");
    // captures_mutable_reference();
    // println!("-----------");
    force_closure_take_ownership();
}
