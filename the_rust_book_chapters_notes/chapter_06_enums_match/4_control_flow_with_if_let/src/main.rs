// If you feel like the match sysntas is to verbose, it is possible to use the `if let` syntax to
// do the same thing. It is even possible to add an `else` statemet to handle default values that
// would be treated in the `other` or `_` clauses of the match.

fn test_match_with_option() {
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum value is {:#?}", max),
        _ => (), // just ignore the None option with symbol `_`.
    }
}

fn test_if_let_alternative(config_max: Option<u8>) {
    if let Some(max) = config_max {
        println!("The maximum value is {:#?}", max);
    } else {
        println!("Not a valid number");
    }
}

fn main() {
    test_match_with_option();
    test_if_let_alternative(Some(3));
    test_if_let_alternative(None);
}
