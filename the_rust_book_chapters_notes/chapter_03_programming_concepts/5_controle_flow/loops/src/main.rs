fn main() {
    // Rust has 3 types of loops: Loop, while and for.
    test_loop();
    test_loop_with_label();
    test_loop_with_return();
    test_while();
    test_for();
    testin_the_rev_expression();
}


fn test_loop() {
    println!("\n testing the loop ");
    // The loop instruction executes forever until we inform for it to finish the loop.
    let mut count: u8 = 0;
    loop {
        println!("Count: {}", count);

        // we break the loop when we get to the last possible value for a u8 signed integer.
        if count == 255 {
            break;
        }

        count = count + 1;
    }
}


fn test_loop_with_label() {
    println!("\n testing the loop with label.");
    let mut count = 0;
    'counting_up: loop {  // this is a label fo the loop named counting_up.
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {  // this condition breaks the outer loop.
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}


fn test_loop_with_return() {
    println!("\n testing the loop with return.");
    // its possible to return the result of a break condition in a loop expression:
    let mut counter: u8 = 0;
    let result: u8 = loop {
        if counter == 255 {
            break counter / 2;
        }
        counter += 1;
    };
    println!("result = {}", result);
}


fn test_while() {
    println!("\n testing the while loop.");
    let mut count: u8 = 0;
    while count < 10 {
        println!("Count = {}", count);
        count += 1;
    }
}


fn test_for() {
    println!("\n testing the for loop to run through a collection.");
    let values: [u8; 5] = [10, 20, 30, 40, 50];
    println!("[");
    for element in values {
        println!("\t{},", element);
    }
    println!("]");
}


fn testin_the_rev_expression() {
    println!("\n testing the rev expression.");
    for number in (1 .. 5).rev() {
        println!("{}!", number);
    }
}
