#[derive(Debug)]
struct Car{
    mpg: u32,
    color: String,
    top_speed: u32,
}

impl Car {
    fn set_mpg(&mut self, mpg: u32){
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: &str) {
        self.color = color.to_lowercase();
    }

    fn set_top_speed(&mut self, top_speed: u32) {
        self.top_speed = top_speed;
    }
}

fn main() {
    let mut car = Car{mpg: 50, color: "red".to_string(), top_speed: 100};
    car.set_mpg(30);
    car.set_color("blue");
    car.set_top_speed(10);
    println!("{:#?}", car);
}

// what can be improved:
// * Implement a default initialization
// * implement the Display for print out the object
// * add unit tests
