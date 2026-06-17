use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals = HashMap::<&str, Vec<String>>::new();
    sauces_to_meals.insert(
        "ketchup",
        vec![String::from("french fries"), String::from("burguers")],
    );

    sauces_to_meals.insert(
        "mayonnaise",
        vec![
            String::from("sandwiches"),
            String::from("burgers"),
            String::from("Coleslaw"),
        ],
    );
    sauces_to_meals.insert(
        "mustard",
        vec![String::from("hot dot"), String::from("Pretzels")],
    );
    println!("{:#?}", sauces_to_meals);

    let result = sauces_to_meals
        .remove("mayonnaise")
        .expect("Value not found");
    println!("Result: {:#?}", result);

    // let result = sauces_to_meals
    //     .remove("mayonnaise")
    //     .expect("Value not found");
    // println!("Result: {:#?}", result);

    println!("{:#?}", sauces_to_meals.get("mustard"));
    println!("{:#?}", sauces_to_meals);

    sauces_to_meals
        .entry("soy sauce")
        .or_insert(vec![String::from("sushi"), String::from("Dumplings")]);
    println!("{:#?}", sauces_to_meals);
}
