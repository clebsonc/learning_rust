fn main() {
    // Instead of giving the ownership to a variable or a function we can pass its reference with
    // the operator &.
    let mut s = String::from("Hello");
    let size = receive_reference(&s);
    println!("The string `{}` has size of `{}`.", s, size);

    modify_reference(&mut s); // we pass a mutable reference for variable s
    println!("The string is now changed: `{}`", s);

    prevent_data_race_example_1();
    // prevent_data_race_example_2();
    prevent_data_race_example_3();

    // dangle_reference();
    let not_dangled: String = no_dangled_reference();
    println!("{:#?}", not_dangled);
}

fn receive_reference(value: &String) -> usize {
    // value is a reference to variable s fro mthe main function.
    println!("{:#?}", value);
    value.len()
    // when this function ends, nothing is dropped, since values does not have a permission to clean
    // the memory from the heap. Who does this is the main function, since the owner of this memory
    // space is the variable `s`.
}

fn modify_reference(word: &mut String) {
    // receives a mutable reference of type String
    word.push_str(" World!");
}


fn prevent_data_race_example_1() {
    // rust can prevent data races at compile time, since it is not possible to have more than one
    // mutable reference at the same time for the same data:
    let mut s = String::from("Hello");
    let r1 = &mut s;
    // let r2 = &mut s;   // this won't work because it is not possible to have 2 or more mutable
    // references to an mutable variable.
    println!("{}", r1);
}


// fn prevent_data_race_example_2() {
//     // rust can prevent data races at compile time, since it is not possible to have a mutable
//     // reference to an imutable object
//     let mut s = String::from("Hello");
//     println!("{}", s);
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;  //this won`t  work because neither the reference of r1 and r2 was used yet.
//                       //Comment this line out and change s to be not mutable.
//     println!("{} {}", r1, r2);
// }

fn prevent_data_race_example_3() {
    // rust can prevent data races at compile time, since it is not possible to have a mutable
    // reference to an imutable object
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
}


// fn dangle_reference() -> &String {
    // this function is saying we are trying to return a reference
    // to a string. However, when the scope of the function is finished, the reference e dropped,
    // which will cause an error.
//    let s = String::from("Hello");
//    return &s
// }  // here the drop function will delete the reference for `s`.


fn no_dangled_reference() -> String {
    let s: String = String::from("String that will not be dangled");
    s
}
