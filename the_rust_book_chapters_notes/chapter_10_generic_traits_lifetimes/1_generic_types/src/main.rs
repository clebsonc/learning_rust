use::std::cmp::Ordering;

// in rust it is possible to define generic data types in the function definition. The generic type
// is placed in the same space of the data type in the function. For example, the function largest
// bellow is capable of finding the largest generic data type T. This type T will work for any
// type that implements the `std::cmp::PartialOrd` trait.


fn largest_item<T: std::cmp::PartialOrd>(values: &[T]) -> &T {
    let mut largest: &T = &values[0];

    for v in values {
        if v > largest {
            largest = v;
        }
    }
    largest
}


// It is also possible to define generic data types for structs and enums:
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle<T> {
    width: T,
    height: T, 
}

impl<T> Rectangle<T> {
    fn dimension(&self) -> (&T, &T) {
        (&self.width, &self.height)
    }
}

impl<T: std::cmp::PartialEq> std::cmp::PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.width == other.width && self.height == other.height {
            return true;
        }
        false
    }
}



impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for Rectangle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.height == other.height && self.width == other.width {
            return Some(Ordering::Equal);
        } else if self.height < other.height && self.width < other.width {
            return Some(Ordering::Less);
        } else {
            return Some(Ordering::Greater);
        }
    }
}




fn main() {
    let ex1 = vec![1, 3, 2, 5, 4];
    let ex2 = vec!['a', 'c', 'b', 'e', 'd'];
    let ex3 = vec![2.0, 3.2, 1.3, 5.8, 4.0];
    println!("Largest Number: {:?}", largest_item(&ex1));
    println!("Largest Char: {:?}", largest_item(&ex2));
    println!("Largest Float: {:?}", largest_item(&ex3));

    let ex4 = Rectangle {
        width: 40,
        height: 30,
    };
    let ex5 = Rectangle {
        width: 41,
        height: 31,
    };
    let ex6 = Rectangle {
        width: 'a',
        height: 'c',
    };
    println!("Rectangle size: {:#?}", ex4);
    println!("Rectangle size: {:#?}", ex6);
    println!("Rectangle 4 Dimension: {:#?}", ex4.dimension());
    println!("Rectangle 5 Dimension: {:#?}", ex6.dimension());

    let ex7: Vec<Rectangle<i32>> = vec![ex5, ex4];
    println!("Largest Rectangle: {:#?}", largest_item(&ex7));
}
