fn main() {
    let instruments = ["guitar", "bass", "drums"];

    let selection = instruments.get(2);

    play_instrument(selection);
    see_instrument(selection);
    println!("Instrument: {:#?}", selection);
}

fn play_instrument(instrument_type: Option<&&str>) {
    match instrument_type {
        Some(x) => println!("Playing with {}", (*x)),
        None => println!("Just Singing..."),
    };
}

fn see_instrument(instrument_type: Option<&&str>) {
    // let is_valid = instrument_type.unwrap();  // this will fail
    let is_valid = instrument_type.expect("Should be a valid instrument.");
    println!("Seeing instrument: {:#?}", is_valid);
}
