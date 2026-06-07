pub const PERSONAL_TRAINER: &str = "Will weight";

pub fn ask_about_program() {
    println!("The weightliftning trainer is {}", PERSONAL_TRAINER);
}

#[derive(Debug)]
pub struct Exercise {
    name: String,
    reps: u32,
}

impl Exercise {
    pub fn new(name: String, reps: u32) -> Self {
        Self {
            name: name,
            reps: reps,
        }
    }
}
