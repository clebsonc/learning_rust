#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Blue,
    Red,
}


struct Inventory {
    shirts: Vec<ShirtColor>,
}


impl Inventory {
    fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        let preference: ShirtColor = user_preference.unwrap_or_else(|| self.most_stocked());

        self.update_shirts(&preference);

        preference
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut blue: u32 = 0;
        let mut red: u32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1,
            }
        };
        if blue >= red {
            return ShirtColor::Blue;
        }
        ShirtColor::Red
    }

    fn update_shirts(&mut self, preference: &ShirtColor) {
        let mut position: Option<usize> = None;
        for (index, color) in self.shirts.iter().enumerate() {
            if color == preference {
                position = Some(index);
                break;
            }
        }
        self.shirts.swap_remove(position.unwrap_or(0));
    }
}

fn main() {
    let mut store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
