fn main() {
    // Instead of giving the ownership to a variable or a function we can pass its reference with
    // the operator &.
    let mut s = String::from("Hello");
    let size = receive_reference(&s);
    println!("The string `{}` has size of `{}`.", s, size);

    modify_reference(&mut s); // we pass a mutable reference for variable s
    println!("The string is now changed: `{}`", s);

    // rust can prevent data races at compile time, since it is not possible to have more than one
    // mutable reference at the same time for the same data:
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
}

fn receive_reference(value: &String) -> usize {
    // value is a reference to variable s fro mthe main function.
    value.len()
    // when this function ends, nothing is dropped, since values does not have a permission to clean
    // the memory from the heap. Who does this is the main function, since the owner of this memory
    // space is the variable `s`.
}

fn modify_reference(word: &mut String) {
    // receives a mutable reference of type String
    word.push_str(" World!");
}
