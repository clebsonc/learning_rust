use std::fmt::DebugStruct;

mod flight_data {

    #[derive(Debug)]
    pub struct Flight {
        origin: String,
        destination: String,
        price: f64,
        passengers: u32,
    }

    impl Flight {
        pub fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
            Self {
                origin,
                destination,
                price,
                passengers,
            }
        }

        pub fn increase_price(&mut self, percentage: f64) -> Result<f64, PercentageError> {
            if percentage < 0. || percentage > 1.0 {
                Err(PercentageError::InvalidPercentage(String::from(
                    "The percentage should be bethween 0 and 1",
                )))
            } else {
                self.price = self.price * (1.0 + percentage);
                Ok(self.price)
            }
        }
        pub fn itinerary(&self) {
            println!("{} -> {}", self.origin, self.destination);
        }

        pub fn total_price(&self) -> f64 {
            return self.price * self.passengers as f64;
        }
    }

    #[derive(Debug)]
    pub enum PercentageError {
        InvalidPercentage(String),
    }
}

fn main() {
    use crate::flight_data::Flight;

    let mut my_flight = Flight::new(String::from("BH"), String::from("SP"), 1234.49, 3);

    let res = my_flight.increase_price(1.23);
    match res {
        Ok(val) => println!("Price increased vay {val}"),
        Err(e) => println!("Error: {:#?}", e),
    }
    println!("Your flight is: {:#?}", my_flight);

    my_flight.itinerary();
    let total_price = my_flight.total_price();
    println!("Total Price: {}", total_price);
}
