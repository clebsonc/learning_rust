#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestations: bool,
}

impl Restaurant {
    pub fn chef_special(&self) -> Option<Food> {
        match self.has_mice_infestations {
            true => None,
            false => {
                let food = if self.reservations < 12 {
                    Food {
                        name: String::from("Uni Sashimi"),
                    }
                } else {
                    Food {
                        name: String::from("Strip Steak"),
                    }
                };
                Some(food)
            }
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if !self.has_mice_infestations {
            if !address.is_empty() {
                Ok(Food {
                    name: String::from("Burger"),
                })
            } else {
                Err(String::from("Address is empty."))
            }
        } else {
            Err(String::from("We have a mice problem."))
        }
    }
}

fn main() {
    let rest1 = Restaurant {
        reservations: 12,
        has_mice_infestations: true,
    };
    let dish = {
        match rest1.chef_special() {
            None => String::from("Empty"),
            Some(a) => a.name,
        }
    };

    println!("dish: {:?}, dish: {}", rest1, dish);
}
