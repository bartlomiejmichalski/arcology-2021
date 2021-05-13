use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct ExperimentDelta {
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

#[macro_export]
macro_rules! percentage {
    ($percent: expr, $value: expr) => {
        $value * ($percent as f32 / 100.00)
    }
}

#[macro_export]
macro_rules! change_property {
    (Time => $value: expr, $builder: expr) => {
        $builder.time($value);
    };
    (Score => $value: expr, $builder: expr) => {
        $builder.score($value);
    };
    (FodQuantity => $value: expr, $builder: expr) => {
        $builder.food_quantity($value);
    };
    (SocialCapital => $value: expr, $builder: expr) => {
        $builder.social_capital($value);
    };
    (Waste => $value: expr, $builder: expr) => {
        $builder.waste($value);
    };
    (Production => $value: expr, $builder: expr) => {
        $builder.production($value);
    };
    (FoodCapacity => $value: expr, $builder: expr) => {
        $builder.food_capacity($value);
    };
    (ArcologyIntegrity => $value: expr, $builder: expr) => {
        $builder.arcology_integrity($value);
    };
    (Population => $value: expr, $builder: expr) => {
        $builder.population($value);
    };
    (PopulationCapacity => $value: expr, $builder: expr) => {
        $builder.population_capacity($value);
    };
}
#[macro_export]
macro_rules! create_delta {
    { $($key:ident => $value:expr),* } => {
        {
            let mut builder = ExperimentDelta::builder();
            $(
                crate::change_property!($key => $value, builder);
            )*
            builder.build()
        }  
     };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_delta_should_create_delta_with_time_increment_1() {
        let delta = create_delta!(Time => 1).unwrap();
        assert_eq!(delta.time, 1);
    }
}