use actix_web::{get, post, Result, web};
use derive_more::{Display};
use serde::{Deserialize, Serialize};

use crate::SharedData;
use crate::r2d2_pool;
use crate::model::Experiment;

use r2d2_pool::{save_experiment, get_experiment};

#[derive(Debug, Display, derive_more::Error)]
#[display(fmt = "Response error: {}", name)]
pub struct ResponseError {
    name: &'static str,
}

impl actix_web::error::ResponseError for ResponseError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub user_id: String,
    pub parameter: Option<String>
}
#[get("/")]
pub async fn ping() -> Result<String, ResponseError> {
    Ok(String::from("Pong"))
}

#[get("/{id}")]
pub async fn retrieve(data: web::Data<SharedData>, web::Path(id): web::Path<String>) -> Result<String, ResponseError> {
    let db_id = format!("{}", id);
    let experiment: Experiment = get_experiment(&data.pool, &db_id)
    .map_err(|_e| ResponseError { name: "Ups !! No set :)"})?;
    let serialized = serde_json::to_string(&experiment).unwrap();
    Ok(serialized)
}
#[post("/")]
pub async fn command(data: web::Data<SharedData>, received_command: web::Json<Command>) -> Result<String, ResponseError> {
    let db_id = format!("{}", received_command.user_id);
    // 1. Create if not exists, load if exists
    let experiment: Experiment = get_experiment(&data.pool, &db_id)
        .map_err(|_e| ResponseError { name: "Ups !! No set :)"})?;
    
    // 2. execute command based on name
    // 3. execute upkeep
    // 4. save to db
    save_experiment(&data.pool, &db_id, &experiment)
        .map_err(|_e| ResponseError { name: "Ups !! No set :)"})?;
    
    Ok(String::from("Ok"))
}