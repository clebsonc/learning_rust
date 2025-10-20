mod cardio;
mod weightlifting;

mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() -> () {
        println!("The nutritionist is {}", crate::diet::NUTRITIONIST);
    }
}

use crate::cardio::CardioTool;
use crate::cardio::Exercise as CardioExercise;
use crate::weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        crate::diet::ask_about_program();
        crate::cardio::ask_about_program();
        crate::weightlifting::ask_about_program();

        Self {
            cardio: CardioExercise::new(String::from("Monday"), CardioTool::Bike, 30),
            weightlifting: WeightliftingExercise::new("Short Abs", 12),
        }
    }
}
