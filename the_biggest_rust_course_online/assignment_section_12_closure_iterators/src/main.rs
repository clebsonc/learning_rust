fn with_closure(values: &[i32]) {
    let iter = values.iter();
    let result: Vec<i32> = iter.map(|x| x * 10).collect();

    println!("with closure: {:?}", result);
}

fn with_some_let(values: &[i32]) {
    let mut iter = values.iter();
    let mut result: Vec<i32> = Vec::<i32>::new();
    while let Some(x) = iter.next() {
        result.push(x * 10);
    }
    println!("with some let: {:?}", result);
}

fn using_for_and_iter(values: &[i32]) {
    let mut result = Vec::<i32>::new();
    for val in values.iter() {
        let i = *val * 10;
        result.push(i);
    }
    println!("with for loop {:?}", result);
}

fn main() {
    // when using [1, 3, 5, 7, 9] we are alocating an ARRAY on the stack.
    // when using vec![1, 3, 5, 7, 9] we are alocating a Vector, which can grow in size. Therefore
    // on the heap.

    let values = [1, 3, 5, 7, 9];
    println!("values: {:?}", values);
    println!("-------------");
    with_closure(&values);
    println!("-------------");
    with_some_let(&values);
    println!("-------------");
    using_for_and_iter(&values);

    // creating a closure. Closures are just like Lambda functions in python.
    let closure_example = |x: i32| x.pow(2);
    println!("{:?}", closure_example(10));

}
