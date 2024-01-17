use std::fs::File;
use std::io::{BufReader, BufRead};


fn open_file() {
    let buffer = File::open("example.txt");
    let file = match buffer {
        Ok(file) => file,
        Err(err) => panic!("File does not exist. {}", err)
    };
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        let res = reader.read_line(&mut line);
        let len = match res {
            Ok(len) => len,
            Err(err) => panic!("Error reading line. {}", err)
        };
        if len > 0 {
            print!("Line: {}", line);
            line.clear();
        }
        else {
            break;
        }
    }
}


fn main() {
    open_file();
}



// Improved version.
// use std::fs::File;
// use std::io::{self, BufReader, BufRead};
// use std::path::Path;
// 
// fn open_file<P: AsRef<Path>>(filename: P) -> io::Result<()> {
//     let file = File::open(filename)?;
//     let reader = BufReader::new(file);
// 
//     for line in reader.lines() {
//         match line {
//             Ok(line) => println!("Line: {}", line),
//             Err(err) => eprintln!("Error reading line: {}", err),
//         }
//     }
// 
//     Ok(())
// }
// 
// fn main() {
//     if let Err(err) = open_file("example.txt") {
//         eprintln!("File does not exist: {}", err);
//     }
// }
