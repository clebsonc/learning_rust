use std::collections::HashMap;


fn main() {
    let mut mymap: HashMap<i32, i32> = HashMap::new();
    mymap.insert(1, 10);
    mymap.insert(2, 30);

    println!("{:#?}", mymap);

    for key in mymap.keys() {
        let value: Option<&i32> = mymap.get(&key);
        match value {
            Some(value) => println!("Found value {value}"),
            None => println!("No value found."),
        }
    }
}
