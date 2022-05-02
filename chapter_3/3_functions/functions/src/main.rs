fn main() {
    // Functions are declared with keyword `fn`. They can be in any place.
    // A function can have parameters that allow to receive arguments when calling its name.
    another_function(5);

    // multiple parameters in functions
    multiple_parameters(5, "hello");

    // In rust there is difference between statements and expressions.
    // Statements do not have a return value, whilst expressions always return a value.
    let number = 5; // this is a statement.

    // the new scope block within curly braces is a expression that returns the value 6 + 5.
    let another_number = {  // starting the new scope.
        let var = 6;

        // Expressions doesn`t have a semicolon, since we are returning its value representation.
        var + number
    };  // ending the scope
    println!("Statement: {} \nExpression: {}", number, another_number);

    // Functions can return value.
    let returned = returning_value(5);
    println!("Returned Value: {}", returned);
}

// Every function parameter must have its type definition annotated.
fn another_function(parameter1: i32) {
    println!(
        "This function receives a single parameter, its value is: {}",
        parameter1
    );
}

// Function receiving two parameters.
fn multiple_parameters(value: u32, greeting: &str) {
    println!("Value: {} - Greeting: {}", value, greeting);
}

// When returning a value, a function must define its type with the arrow indicator. Ex:
fn returning_value(value: u32) -> u32 {
    // this is a expression, if we add a semicolon at the end it will considered a statement, and
    // the compiler will fail describing that the function must return an unsegned integer.
    value * value
}


