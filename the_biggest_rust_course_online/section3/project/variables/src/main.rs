fn main() {
    let val: i32 = 1_337;
    let small_value = val as i16;
    let fraction = 1.3435436;
    let with_milk: bool = true;
    let with_sugar: bool = false;

    let is_my_type_of_coffe: bool = if with_milk && with_sugar { true } else { false };
    let is_acceptable_coffe: bool = if with_milk || with_sugar { true } else { false };

    let array: [i8; 4] = [8, 10, 12, 14];
    let all_together = (val, fraction, is_acceptable_coffe, array);
    println!("val: {val}\nsmall val: {small_value}\nfraction: {fraction:.3}");
    println!("type of coffee: {is_my_type_of_coffe}");
    println!("acceptable coffee: {is_acceptable_coffe}");
    println!("Array: {:?}", array);
    println!("All together: {:#?}", all_together);
}
