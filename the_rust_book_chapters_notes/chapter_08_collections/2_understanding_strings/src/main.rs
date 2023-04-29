// strings in rust are encoded with UTF-8. This means that it is possible to
// have special characters, like the "Зд" or "नमस्ते"

// When representing special charactes, not all of them have the same byte
// size. In rust was decided to represent the strings as a sequence of bytes
// when indexing.

// Therefore, when there is a string like "Зд", there won`t be an &index[0]=="З",
// but its byte representation. Considering that each letter above takes 2
// bytes, there will be a total of 4 bytes allocated for these two letters.
// Therefore, the indexing for these two letteers would be &index[0 .. 4].


// In order to actualy get the representation for the characters it is
// necessary to use the special function `chars` or `bytes`. It is not advised
// to use the indexing with bytes, such as &index[0 .. 4]

fn main() {
    let mystr: &str = "hello world";

    println!("{mystr}");

    let chars = mystr.chars();
    for c in chars {
        print!("{c}-");
    }

    let mystr: String = String::from("hello world");
    println!("{:#?}", &mystr[0 .. 3]);
 

    // testing bytecodes.
    for b in mystr.bytes() {
        print!("{b}-");
    }
}
