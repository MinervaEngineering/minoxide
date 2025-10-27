use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::games::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGameDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateGameDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateGameDto> for ActiveModel {
    fn from(dto: CreateGameDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateGameDto> for ActiveModel")
    }
}

impl UpdateGameDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateGameDto::apply_to")
    }
}
