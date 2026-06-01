#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Flight {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn changes_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn increase_price(&mut self) {
        self.price = self.price * 1.20;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut flight = Flight::new(String::from("BH"), String::from("RJ"), 345.0, 2);
    flight.changes_destination(String::from("SP"));
    flight.increase_price();
    flight.itinerary();

    let new_flight = Flight {
        origin: String::from("Salvador"),
        destination: String::from("Goias"),
        ..flight
    };
    println!("Flight: {:#?}", flight);
    println!("New Flight: {:#?}", new_flight);
}
