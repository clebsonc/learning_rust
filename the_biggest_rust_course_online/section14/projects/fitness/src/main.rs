use fitness::diet;
use fitness::weightlifting;

fn main() {
    println!("Nutritionist: {}", diet::NUTRITIONIST);
    diet::ask_about_program();

    println!("--------------");
    weightlifting::ask_about_program();

    let exercise = weightlifting::Exercise::new(String::from("Pull ups"), 10);
    println!("{:#?}", exercise);
}
