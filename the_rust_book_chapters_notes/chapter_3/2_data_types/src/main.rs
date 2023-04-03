fn main() {
    integers_examples();
    floats_examples();
    numeric_operations();
    boolean_example();
    character_example();

    // Compound types are Tuples and arrays.
    tuple_example();
    array_example();
}


fn integers_examples() {
    // Rust has 4 scalar types: integers, floating-point, numbers, booleans, and characters.
    // Integers can have:

    // Length  | Signed                                  | Unsigned                               |
    // 8-bit   | i8 [-2^(8-1), 2^(8-1)-1]                | u8 [0, 2^8-1]                          |
    // 16-bit  | i16 [-2^(16-1), 2^(16-1)-1]             | u16 [0, 2^16-1]                        |
    // 32-bit  | i32 [-2^(32-1), 2^(32-1)-1]             | u32 [0, 2^32-1]                        |
    // 64-bit  | i64 [-2^(64-1), 2^(64-1)-1]             | u64 [0, 2^64-1]                        |
    // 128-bit | i128 [-2^(128-1), 2^(128-1)-1]          | u128 [0, 2^128-1]                      |
    // arch    | isize architecture type (32 or 64 bits) | usize architecture type (32 or 64 bits)|

    // Example of unsigned variable 32 bits.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess: {}", guess);

    // variables can overflow. Example an unsigned integer 8 bits can hold maximum of 255.
    // If compiled in debug mode a `panic` occur. If compiled in release mode an overflow occur.
    // Actually, when I compiled with flag --release it did not allowed the overflow with a value
    // bigger than 255.
    let overflow: u8 = 255;
    println!("Value of overflow: {}", overflow);
}


fn floats_examples() {
    // floating-point can have type `f32` and `f64`, which are 32 and 64 bits in size. Default is
    // f64. All floating points are signed. Example of declaration.
    let x = 2.0; // f64 by Default
    let y: f32 = 3.0; // this is how we represent a float number with 32 bits.
    println!("Float number with 64 bits: {}", x);
    println!("Float number with 32 bits: {}", y);
}


fn numeric_operations() {
    // rust support the basic mathematical operatiosn: addition, subtraction, multiplication, division, and remainder
    let sum = 5 + 10;
    let difference = 93.4 - 23.0;
    let product = 20.0 * 2.5;
    let quotient = 10.0 / 3.0;
    let floored = 2 / 3;
    let remainder = 11 % 3;
    println!("Sum: 5 + 10 = {}", sum);
    println!("difference: 93.4 - 23.0 = {}", difference);
    println!("product: 20.0 * 2.5 = {}", product);
    println!("quotient: 10.0 / 3.0 = {}", quotient);
    println!("floored: 2 / 3 = {}", floored);
    println!("remainder: 11 % 3 = {}", remainder);
}

fn boolean_example() {
    // booleans are 1 byte in size.allow
    let t = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);
}

fn character_example() {
    // characters are the most primitive types in rust. Each character has 4 bytes representing a
    // Unicode Scalar Value. Therefore it can represent more than just ASCII Symbols, for example:
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);
}

fn tuple_example() {
    // Tuples can group values of different types.
    // Once a tuple is created it can not grow in size or change its structure type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple prior destruction: {:?}", tup);  // print all elements of a tuple in a single line
    println!("tuple prior destruction: {:#?}", tup);  // print all elements of a tuple in a structured way

    // Once a tuple is created it is possible to destruct it in different variables. Ex:
    let (x, y, z) = tup;  // x, y, and z have copies of the values hold by the tuple.
    println!("The value of y is: {} , {} , {}", x, y, z);

    // We can also access tuple elements using the `period` operator followed by the index
    // (starting at zero) of the tuple.
    println!("fist: tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
}

fn array_example() {
    // Arrays also hold many elements together. The distinction is that an array must hold all
    // elements of the same type. Arrays allocate values in the stack instead of the heap. The
    // array have a fixed size. Example:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:#?}", a);

    // Its possible to repeat the creation of many elements togeter in an array. Ex:
    let b = [3; 5];
    println!("b: {:#?}", b);

    // It is possible to access array elements by using an index starting at zero insede brackets.
    println!("a[0] = {}, a[2] = {}, a[4] = {}", a[0], a[2], a[4]);

    // Rust protects the access against invalid array index. For example, running the bellow code
    // will cause a panic index.
    // let c = [1, 2, 3, 4];
    // println!("c[10] = {}", c[10]);
}

