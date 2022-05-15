// First lets write a program that computes the area of a rectangle, than
// refactor it to use structs.
fn compute_rectangle_area(width: f64, length: f64) -> f64 {
    let area = width * length;
    area
}

// rectangle area with tuples
fn compute_rectangle_area_with_tuple(dimensions: (f64, f64)) -> f64 {
    let area = dimensions.0 * dimensions.1;
    area
}

// By default it is not possible to print an rectangle structure using the macro
// `println!`. However, we can use the derivative trait debug to print the
// structure.
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// rectangle with structs
fn compute_rectangle_area_with_structs(rectangle: &Rectangle) -> f64 {
    let area: f64 = rectangle.width * rectangle.height;
    area
}

fn main() {
    let length = 3.0;
    let width = 4.0;

    println!("Area {}", compute_rectangle_area(width, length));

    let dimensions: (f64, f64) = (length, width);
    println!("Area {}", compute_rectangle_area_with_tuple(dimensions));

    let rectangle = Rectangle {
        width,
        height: length,
    };
    println!("Area {}", compute_rectangle_area_with_structs(&rectangle));
    println!("Rectangle: {:#?}", rectangle);

    // using the dbg to print line numbers.
    dbg!(&rectangle);
}
