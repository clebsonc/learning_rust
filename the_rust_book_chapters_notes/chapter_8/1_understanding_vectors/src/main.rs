fn main() {
    let v: Vec<i32> =  vec![1, 2, 3, 4, 5];  // create a vector using the macro vec!

    let does_not_exist = &v[4];
    println!("It exists {does_not_exist}");

    let v: Vec<i32> =  vec![1, 2, 3, 4, 5];
    let indexes: &[usize] = &[10, 4];  // this is a slice array from a vector
    for i in indexes {
        let check_existence: Option<&i32> = v.get(*i);
        match check_existence {
            Some(check_existence) => println!("It exists {check_existence}"),
            None => println!("The index is invalid."),
        }
    }

    // It is also possible to create a vector using the Vector struct:
    #[allow(unused_mut)]
    let mut v: Vec<i32> = Vec::new();  // when creating a new vector, its length and capacity is 0.
                                       // The space is only allocated when the first push at the
                                       // vector happens.

    println!("Capacity: {}", v.capacity());  // the capacity and length will be 0
    println!("Length: {}", v.len());

    v.push(10);  // first push happens
    println!("Capacity: {}", v.capacity());  // the capacity and length will change.
    println!("Length: {}", v.len());


}
