use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Experiment {
    time: i32,
    score: f32,
    food_quantity: i32,
    waste: i32,
    social_capital: i32,
    production: i32,
    food_capacity: i32,
    arcology_integrity: i32,
    population: i32,
    population_capacity: i32
}

impl Experiment { 
    pub fn new() -> Experiment {
        Experiment {
            time: 1,
            score: 0.0,
            food_quantity: 0,
            waste: 0,
            social_capital: 0,
            production: 0,
            food_capacity: 0,
            arcology_integrity: 0,
            population: 0,
            population_capacity: 0
        }
    }
}