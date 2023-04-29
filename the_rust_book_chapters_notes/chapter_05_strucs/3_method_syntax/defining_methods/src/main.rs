// Methods are just like functions, but are used in the context of Structs,
// enums and traits.
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Methods for strucs are defined within the struct context. And this is done by
// using the `impl` command:
impl Rectangle {
    // implementation for rectangle! All functions defined here are called
    // associated functions.

    // the first parameter for a method must be the command self!
    // In this example it is used &self because we don`t need to take ownership
    // of the values, just borrow their reference.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // passing more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // if we create a method without the self parameter it will be treated like
    // a regular function. We must access it using the namespace syntax: `::`
    fn square(size: f64) -> Rectangle {
        Self {  // here `Self` is an alias for `Rectangle`, we can use both syntax.
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 6.0,
        height: 5.4,
    };
    let rect2 = Rectangle {
        width: 7.0,
        height: 6.4,
    };
    let rect3 = Rectangle {
        width: 3.0,
        height: 2.4,
    };
    println!("Rectangle {:#?}", rect1);
    println!("Rectangle area: {:#?}", rect1.area());

    println!("Rectangle 1 can hold rectangle 2: {}", rect1.can_hold(&rect2));
    println!("Rectangle 1 can hold rectangle 3: {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3.0);  // The :: means that we are accessing the function square
                                          // in the namespace Rectangle.
    println!("Square: {:#?}", square);
}
