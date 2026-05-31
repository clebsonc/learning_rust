fn main() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];
    let mid_pos = cereals.len() / 2;
    let last_three_pos = cereals.len().saturating_sub(3);

    let first_two = &cereals[0..2];
    println!("First Two: {:#?}", first_two);

    let middle = &cereals[mid_pos];
    println!("Middle: {:#?}", middle);

    let last_three = &mut cereals[last_three_pos..];

    last_three[2] = String::from("Lucky Charms");
    println!("Last Three: {:#?}", last_three);

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[0..6];
    println!("{cookie}");
    println!("All cereals: {:#?}", cereals);

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("{puffs}");
}
