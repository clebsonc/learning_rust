use std::fmt::Debug;

#[derive(Debug)]
struct Car{
    mpg: u32,
    color: String,
    top_speed: u32,
}

struct Motorcycle {
    mpg: u32,
    color: String,
    top_speed: u32,
}

trait VehicleSetter {
    fn set_mpg(&mut self, mpg: u32);
    fn set_color(&mut self, color: String);
    fn set_top_speed(&mut self, top_speed: u32);
}

impl VehicleSetter for Car {
    fn set_mpg(&mut self, mpg: u32){
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color.to_lowercase();
    }

    fn set_top_speed(&mut self, top_speed: u32) {
        self.top_speed = top_speed;
    }
}

impl VehicleSetter for Motorcycle {
    fn set_mpg(&mut self, mpg: u32){
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color.to_lowercase();
    }

    fn set_top_speed(&mut self, top_speed: u32) {
        self.top_speed = top_speed;
    }
}


fn print<T: Debug>(parameter: T){
    println!("{:#?}", parameter);
}

impl Debug for Motorcycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Motorcycle: \n\t Color: {} \n\t Top Speed: {}", self.color, self.top_speed)
    }
}

fn main() {
    let mut car = Car{mpg: 50, color: "red".to_string(), top_speed: 100};
    let mut motorcycle = Motorcycle{mpg: 50, color: "red".to_string(), top_speed: 100};
    car.set_mpg(30);
    car.set_color("blue".to_string());
    car.set_top_speed(10);
    motorcycle.set_mpg(1);
    motorcycle.set_color("yellow".to_string());
    motorcycle.set_top_speed(2);
    println!("{:#?}", car);
    println!("{:#?}", motorcycle);
    print(motorcycle);
    print(car);
}

// what can be improved:
// * Implement a default initialization
// * implement the Display for print out the object
// * add unit tests
