#[allow(dead_code)]
enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(&self) -> u8 {
        match self {
            Self::Triangle => 3,
            Self::Square => 4,
            Self::Pentagon => 5,
            Self::Octagon => 8,
        }
    }
}

fn main() {
    let triangle = Shape::Triangle;
    let square = Shape::Square;
    println!("{}", triangle.corners());
    println!("{}", square.corners());
}
