fn eat_meal(meal: &mut String) {
    meal.clear();
}

fn main() {
    let mut is_concert: bool = true;
    let is_event: bool = is_concert;
    is_concert = false;

    let mut sushi: &str = "Salmon";
    let dinner = sushi;
    sushi = "Tilapia";

    // The String type does not allow to change the value because
    let mut fruit = String::from("apple");
    // let lunch = fruit;
    // fruit = "orange";

    println!("Is concert: {is_concert}");
    println!("Is event: {is_event}");

    println!("Sushi: {sushi}, Dinner: {dinner} ");

    eat_meal(&mut fruit);
    println!("Fruit: {fruit}")
}
