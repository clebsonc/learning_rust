use crate::garden::vegetables::Asparagus;

pub mod garden;  // include the code in garden.rs


fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
