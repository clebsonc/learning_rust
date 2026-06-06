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
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation == false {
            if self.reservations < 12 {
                return Some(Food {
                    name: String::from("Uni sahimi"),
                });
            } else {
                return Some(Food {
                    name: String::from("Strip Steak"),
                });
            }
        }
        None
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("There is a mice infestation problem."));
        }

        if address.is_empty() {
            return Err(String::from("No address was provided."));
        }

        Ok(Food {
            name: String::from("Burger"),
        })
    }
}

fn main() {
    let rest = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:#?}", rest.chef_special());
    println!("{:#?}", rest.deliver_burger("123 Elm Street"));
    println!("----");

    let rest = Restaurant {
        reservations: 11,
        has_mice_infestation: false,
    };
    println!("{:#?}", rest.chef_special());
    println!("{:#?}", rest.deliver_burger(""));
    println!("{:#?}", rest.deliver_burger("123 Elm Street"));
    println!("----");

    let rest = Restaurant {
        reservations: 16,
        has_mice_infestation: false,
    };
    println!("{:#?}", rest.chef_special());
    println!("{:#?}", rest.deliver_burger(""));
    println!("{:#?}", rest.deliver_burger("123 Elm Street"));
}
