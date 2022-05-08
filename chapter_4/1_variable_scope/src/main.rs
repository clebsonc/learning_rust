// the scope is the range within a program in which an item is valid
// the string literal in variable s bellow is valid only within the curly brackets

fn main() {
    // this is the scope of the main function

    {
        // this is a new scope
        let s = "hello"; // here `s` is a string literal and it is defined at compile time.
                         // `s` is created on the stack since we know its size at the compiler
                         // time. String literals are immutable, since it is created in the stack.
        println!("{}", s);
        // when this scope finishes, the variable s no longer exists.
    }  // here the `drop` function is callep upon the s string.

    the_string_type();
    ways_variables_interact();
}

fn the_string_type() {
    // Different from string literals, the String type is mutable and it is allocated in the heap.
    // It is basically a pointer in the stack pointing to the heap location in which the data is
    // stored.
    let mut s = String::from("hello"); // creating a String type from the string literal.
    s.push_str(" world!"); // it is possible to add content after the end of the string `s`, with a
                           // string literal this is not possible.
    println!("{}", s);
}

fn ways_variables_interact() {
    // items that are created on the stack can be copied to other variables, whist items that
    // are created on the heap are shallow copied.
    let x = 5;
    let y = x;  // here y is a copy of x, and has its owns space on the stack.
    println!("{} - {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;  // here s2 is a shallow copy from s1. This means that s1 looses its reference on
                  // the heap for the string.
                  // This means that the content of `s1` is moved to s2. After this s1 is invalid.
    println!("{}",  s2);


    // If we want to make a deep copy of a String to another variable, we must clone its content:
    let s3 = s2.clone();  // this means that there is another pointer (s3) on the stack mappinglet
                          // to other position on the heap that is different from what s2 is
                          // pointing to.
    println!("{}", s3);
}
