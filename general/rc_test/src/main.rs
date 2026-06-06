use std::rc::Rc;

fn main() {
    let array = [1, 2, 3, 4];
    // {
    let rc1 = Rc::new(array);
    let rc2 = rc1.clone();
    // }
    // drop(array);
    // println!("RC: {:?}", rc1);
    println!("array: {:?}", array);
    println!("RC2: {:?}", rc2);
}
