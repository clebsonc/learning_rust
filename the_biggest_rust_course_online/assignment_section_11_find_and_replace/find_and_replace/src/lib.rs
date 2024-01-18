use regex::Regex;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn replace(full_text: &str, to_find: &str, to_replace: &str) -> String {
    // panic if invalid
    let re = Regex::new(to_find).unwrap();

    let result = re.replace_all(full_text, to_replace).to_string();
    
    let spaces = Regex::new(r"\s+").unwrap();
    spaces.replace_all(&result, " ").trim().to_string()
}


#[cfg(test)]
mod test{
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
