use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::pitcher_game::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePitcherGameDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePitcherGameDto {
    // TODO: Add fields based on entity definition
}

impl From<CreatePitcherGameDto> for ActiveModel {
    fn from(dto: CreatePitcherGameDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreatePitcherGameDto> for ActiveModel")
    }
}

impl UpdatePitcherGameDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdatePitcherGameDto::apply_to")
    }
}
