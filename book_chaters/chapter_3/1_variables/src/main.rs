fn main() {
    // create a mutable variable.
    let mut x = 5;
    println!("The value of x is: {}", x);

    // since x is a mutable variable, we can change its value. 
    x = 6;
    println!("The value of x is: {}", x);

    // constants can`t be modified aftewards. Once they are created they can never be changed.
    // constants use the keyworkd `const`.
    const ONE_MINUTE_IN_SECONDS: u32 = 60;
    const ONE_HOUR_IN_SECONDS: u32 = ONE_MINUTE_IN_SECONDS * 60;
    const THREE_HOURS_IN_SECONDS: u32 = 3 * ONE_HOUR_IN_SECONDS;

    println!("Hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // we can shadow variables.
    // Consider the following immutable variable.
    // It value can`t change, since it does not have the keyword `mut`.
    let s = 10;
    println!("The value of `s` is {}", s);
    let s = s + 2;  // this variable is shadowing the first variablen `s`.
    println!("The value of `s` is {}", s);

    {
        // this is a inner scope.
        let s = s + 2;  // this variable `s` is shadowing the variable s from the outer scope.
        println!("The value of `s` is {}", s);
    } // this scope is done, the variable s is cleaned up.
    println!("The value of `s` is {}", s);  // at the end of 

    // When shadowing, we can change its data type:
    let spaces = "good";  // here `spaces` is an immutable string.
    println!("This is the value of spaces: {}. As we can see it is a string reference.", spaces);
    let spaces = spaces.len();  // here spaces is a number type
    println!("This is the value of spaces: {}. As we can see it is a number reference.", spaces);

    // the following two lines won`t work, since we can't change the type of spaces. Try it out.
    // let mut spaces = "bad";
    // spaces = spaces.len()  
}
