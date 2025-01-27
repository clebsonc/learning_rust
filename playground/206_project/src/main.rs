#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn get_special_dish(&self) -> Food {
        let mut special_dish = Food {
            name: String::from("Uni Sashimi"),
        };
        if self.reservations >= 12 {
            special_dish = Food {
                name: String::from("Strip Steak"),
            };
        }
        special_dish
    }

    fn chef_special(&self) -> Option<Food> {
        match self.has_mice_infestation {
            false => Option::Some(self.get_special_dish()),
            true => None,
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        match self.has_mice_infestation {
            true => Result::Err(String::from("Soerry we have a mice infestation")),
            false => {
                if address.is_empty() {
                    Result::Err(String::from("No delivery address specified"))
                } else {
                    Result::Ok(Food {
                        name: String::from("Burger"),
                    })
                }
            }
        }
    }
}

fn main() {
    let rest = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("Has mice infestation: {:#?}", rest.chef_special());
    println!("{:#?}", rest.deliver_burger("123 Elm Street"));

    let rest2 = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!("Has mice infestation: {:#?}", rest2.chef_special());
    println!("{:#?}", rest2.deliver_burger(""));
    println!("{:#?}", rest2.deliver_burger("Valid Addres..."));
}
