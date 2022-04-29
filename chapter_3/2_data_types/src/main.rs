fn main() {
    // Rust has 4 scalar types: integers, floating-point, numbers, booleans, and characters.
    // Integers can have:

    // Length  | Signed                                  | Unsigned                               |
    // 8-bit   | i8 [-2^(8-1), 2^(8-1)-1]                | u8 [0, 2^8-1]                          |
    // 16-bit  | i16 [-2^(16-1), 2^(16-1)-1]             | u16 [0, 2^16-1]                        |
    // 32-bit  | i32 [-2^(32-1), 2^(32-1)-1]             | u32 [0, 2^32-1]                        |
    // 64-bit  | i64 [-2^(64-1), 2^(64-1)-1]             | u64 [0, 2^64-1]                        |
    // 128-bit | i128 [-2^(128-1), 2^(128-1)-1]          | u128 [0, 2^128-1]                      |
    // arch    | isize architecture type (32 or 64 bits) | usize architecture type (32 or 64 bits)|

    // Example of unsignet variable 32 bits.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess: {}", guess);

    // variables can overflow. Example an unsigned integer 8 bits can hold maximum of 255.
    // If compiled in debug mode a `panic` occur. If compiled in release mode an overflow occur.
    let overflow: u8 = 255;
    println!("Value of overflow: {}", overflow);

    println!("result of `my_sum` function: {}", my_sum(3, 5));
}

fn my_sum(a: u8, b: u8) -> u8 {
    a + b
}
