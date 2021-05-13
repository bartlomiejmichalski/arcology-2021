use serde::{Deserialize, Serialize};
use super::experiment_delta::ExperimentDelta;

#[derive(Serialize, Deserialize, Debug)]
pub struct Experiment {
    pub time: i32,
    pub score: f32,
    pub food_quantity: f32,
    pub waste: f32,
    pub social_capital: f32,
    pub production: f32,
    pub food_capacity: f32,
    pub arcology_integrity: f32,
    pub population: f32,
    pub population_capacity: f32
}
#[allow(dead_code)]
impl Experiment { 
    pub fn empty() -> Experiment {
        Experiment::new(1, 0.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0)
    }
    pub fn new(time: i32, score: f32, food_quantity: f32, waste: f32, social_capital: f32, 
        production: f32, food_capacity: f32, arcology_integrity: f32, 
        population: f32, population_capacity: f32) -> Experiment {
        Experiment {
            time: time,
            score: score,
            food_quantity: food_quantity,
            waste: waste,
            social_capital: social_capital,
            production: production,
            food_capacity: food_capacity,
            arcology_integrity: arcology_integrity,
            population: population,
            population_capacity: population_capacity
        }
    }
    pub fn apply(&self, delta: ExperimentDelta) -> Experiment {
        Experiment::new(
            self.time + delta.time,
            self.score + delta.score,
            self.food_quantity + delta.food_quantity,
            self.waste + delta.waste,
            self.social_capital + delta.social_capital,
            self.production + delta.production,
            self.food_capacity + delta.food_capacity,
            self.arcology_integrity + delta.arcology_integrity,
            self.population + delta.population,
            self.population_capacity + delta.population_capacity
        ) 
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apply_should_increase_time_by_1() {
        let delta : ExperimentDelta = crate::create_delta!(Time => 1).unwrap();
        let experiment = Experiment::empty();
        let new_experiment = experiment.apply(delta);
        assert_eq!(new_experiment.time, 2);
    }
}