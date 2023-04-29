mod rectangle;

use crate::rectangle::Rectangle;


fn main() {
    let ret: Rectangle = Rectangle {
        width: 10,
        length: 20,
    };

    println!("Area of rectangle: {}", ret.area());
}
