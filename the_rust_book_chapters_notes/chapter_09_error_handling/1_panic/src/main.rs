use ::std::fs::File; // to read and write file.
use::std::io::{self, Write};

fn main() {
    // panic_example();

    match result_example() {
        Ok(ok) => ok,
        Err(e) => println!("Failure while checking example: {:#?}", e),
    }
}

#[allow(dead_code)]
fn panic_example() {
    // it is possible to use the macro `panic!` to to express failure.
    // For example, let`s supose that a the code bellow must exit when a value equals 10.
    let v: Vec<i8> = vec![1, 2, 4, 10, 15];
    for i in v {
        if i == 10 {
            panic!("number not acceptable.");
        }
    }
}

fn result_example() -> Result<(), io::Error> {
    // when dealing with errors it is possible to recover from them.
    // In rust there is the Enum `Result` with two variants: (i) Ok(T) and ii Err(E).
    // For example when reading a file:
    let mut file = match File::open("Foo.txt") {
        Ok(data) => data,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("Foo.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:#?}", e),
            },
            other => panic!("Problem opening the file: {:?}", other),
        },
    };
    
    // it is possible to use a shortcut for the match above, for example, the function `write_all`
    // also returns a `Result`, and instead of using the match as above it is possible to use
    // shortcut `?`.
    // Besides that, it is also possible to use the command `unwrap` and `expect`, where expect
    // allows to give a nicer text message.
    
    // file.write_all(b"Hello, world!")?;
    // file.write_all(b"Hello, world!").unwrap();
    file.write_all(b"Hello, world!").expect("Message explaning the error.");

    Ok(())
}
