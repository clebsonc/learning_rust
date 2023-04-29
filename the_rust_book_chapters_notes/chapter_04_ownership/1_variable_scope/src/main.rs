// the scope is the range within a program in which an item is valid
// the string literal in variable s bellow is valid only within the curly brackets

fn main() {
    // this is the scope of the main function
    println!("Function main!");
    {
        // this is a new scope
        let s = "hello"; // here `s` is a string literal and it is defined at compile time.
                         // `s` is created on the stack since we know its size at the compiler
                         // time. String literals are immutable, since it is created in the stack.
        println!("{}", s);
        // when this scope finishes, the variable s no longer exists.
    } // here the `drop` function is called upon the `s` string, releasing the memory allocated by
      // it.

    the_string_type();
    ways_variables_interact();
    // the semantics for passing value to a function works similar to variables.
    ownership_and_functions();

    // it is possible to return more than one value from fuctions:
    let word = String::from("triangle");
    let (word, len) = compute_length(word);
    println!("String: {} - size: {}", word, len);
    println!("---------------------");
}

fn the_string_type() {
    println!("Function the string type.");
    // Different from string literals, the String type is mutable and it is allocated in the heap.
    // It is basically a pointer in the stack pointing to the heap location in which the data is
    // stored.
    let mut s = String::from("hello"); // creating a String type from the string literal.
    s.push_str(" world!"); // it is possible to add content after the end of the string `s`, with a
                           // string literal this is not possible.
    println!("{}", s);
    println!("---------------------");
}

fn ways_variables_interact() {
    println!("Function ways variable interact");
    // items that are created on the stack can be copied to other variables, whist items that
    // are created on the heap are shallow copied.
    let x = 5;
    let y = x; // here y is a copy of x, and has its owns space on the stack.
    println!("{} - {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // here s2 is a shallow copy from s1. This means that s1 looses its reference on
                 // the heap for the string. This is called move operation.
                 // This means that the content of `s1` is moved to s2. After this s1 is invalid.
    // println!("{}", s1);  // if we attempt to run this print there will be an error, since the
                            // reference for s1 no longer exists
    println!("{}", s2);

    // If we want to make a deep copy of a String to another variable, we must clone its content:
    let s3 = s2.clone(); // this means that there is another pointer (s3) on the stack mapping
                         // to other position on the heap that is different from what s2 is
                         // pointing to. Clone is an expensive operation.
    println!("{}", s3);
    println!("---------------------");
}

fn ownership_and_functions() {
    println!("Function ownership and functions");
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // move the ownership from `s` to `some_string` in function
                        // takes_ownership. Since `gives` its ownership to `some_string`, when the
                        // function is finished it is no longer valid.
    // println!("{:#?}", s); // this print does not work, since `s` no longer has the ownership. It
                             // was already freed at the end of function `takes_ownership`.

    // scalar types do not move ownership, since it is created on the stack.
    let i = 50;
    sum_value(i);
    println!("{:#?}", i);
    // It is ok to call `i` here, since it does not give ownership of its content to function `sum_value`.

    // if we want to retrieve the ownership back, the function must return it.
    let mut value = String::from("Hello");
    value = takes_and_gives_back(value);
    // now we can use value again:
    println!("{}", value);
    println!("---------------------");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // when the function finishes the `drop` function is called and free the memory for `some_string`.

fn sum_value(value: u8) {
    println!("{}", value + 10);
}

fn takes_and_gives_back(mut receiving: String) -> String {
    receiving.push_str(" World!");
    receiving   // this is the returned value
}

fn compute_length(word: String) -> (String, usize) {
    let length = word.len();
    (word, length)
}
