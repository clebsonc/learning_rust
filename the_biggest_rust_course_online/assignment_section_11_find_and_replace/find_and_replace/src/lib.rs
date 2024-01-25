use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use text_colorizer::Colorize;

/// Simple program to find and replace text in file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the input file to find text
    #[arg(short, long)]
    pub input: String,

    /// Name of the output file with replaced data
    #[arg(short, long)]
    pub output: String,

    /// Text that will be replaced
    #[arg(short, long)]
    pub find: String,

    /// New text that will replace the pattern
    #[arg(short, long)]
    pub replace: String,
}

pub fn replace(full_text: &str, to_find: &str, to_replace: &str) -> String {
    // panic if invalid
    let re = Regex::new(to_find).unwrap();

    let count_matches = re.find_iter(full_text).count();
    println!(
        "Number of found matches for {} is {}. They are all going to be replaced for {}",
        to_find.red(),
        count_matches.to_string().blue(),
        to_replace.green(),
    );
    let result = re.replace_all(full_text, to_replace).to_string();

    let spaces = Regex::new(r"\s+").unwrap();
    spaces.replace_all(&result, " ").trim().to_string()
}

pub fn open_file(path: &String) -> String {
    let p = Path::new(path);
    let mut f = File::open(p).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    data
}

pub fn save_to_file(path: &String, data: &String) {
    let p = Path::new(path);
    let rfc = File::create(p);
    let mut f = match rfc {
        Ok(f) => {
            println!("Output file is correctly created: {}", path.blue());
            f
        }
        Err(e) => panic!(
            "The file was not created as expected: {}. Error message: {}",
            path.blue(),
            e
        ),
    };
    f.write_all(data.as_bytes()).unwrap();
}

#[cfg(test)]
mod test {
    use super::replace;

    #[test]
    fn test_replace_single_word() {
        let text = String::from("Hello, have a good night pretty lady.");
        let to_find = "pretty";
        let to_replace = "dear";

        let expected = String::from("Hello, have a good night dear lady.");
        let text = replace(&text, to_find, to_replace);

        assert_eq!(expected, text);
    }

    #[test]
    fn replace_empty_word() {
        let text = String::from("Hello, have a good night pretty lady.");
        let to_find = "pretty";
        let to_replace = "";

        let expected = String::from("Hello, have a good night lady.");
        let text = replace(&text, to_find, to_replace);

        assert_eq!(expected, text);
    }
}
