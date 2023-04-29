// the match control flow allow to compare a value against a series of patterns.
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // the match expression evaluates each 'arm' and returns its value.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Arms can have any code inside curly brackets.");
            10
        }
        Coin::Quarter => 25,
    }
}

fn testing_first_match_expression() {
    let coin: Coin = Coin::Dime;
    println!("{}", value_in_cents(coin));
}

// It is possible to bind patterns to values in a match expression. Consider for example this enum:
#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
}

#[allow(dead_code)]
#[derive(Debug)]
enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_quarter_having_state(coin: CoinWithState) -> u8 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn testing_match_that_binds_patterns_to_value() {
    let coin = CoinWithState::Quarter(UsState::Arizona);
    value_in_cents_with_quarter_having_state(coin);
}




// When we use Option in match expression is necessary we cover both the None and Some clause.
// The `match` clause is exaustive, if we do not cover all possibilites the compiler will know
// that there are things missing.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// when using a match, is possible to add a default action for values that we do not need specific
// actions. This is done with the `other` clause.
fn roll_the_dice(dice_number: u8) -> String {
    match dice_number {
        3 => String::from("Add fancy hat!"),
        7 => String::from("Remove fancy hat!"),
        other => {  // this pattern is added for last, because clauses in `match` are evaluated by
                    // order. If we don`t  want to take action for the default value it is
                    // possible to use the symbol `_`. This symbol `_` basically tells the compiler that we
                    // don`t care with the default value, and we should not handle it.
            let number_string = other.to_string();
            let sentence = String::from("Times to move: ");
            format!("{}{}", sentence, number_string)
        }
    }
}

fn testing_default_values_in_match() {
    let message: String = roll_the_dice(1);
    println!("Dice game message: {}", message)
}

fn testing_matching_with_option() {
    let five: Option<i32> = plus_one(Some(5));
    let six: Option<i32> = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);
}

fn main() {
    testing_first_match_expression();
    println!("-----------------------");
    testing_match_that_binds_patterns_to_value();
    println!("-----------------------");
    testing_matching_with_option();
    println!("-----------------------");
    testing_default_values_in_match();

}
