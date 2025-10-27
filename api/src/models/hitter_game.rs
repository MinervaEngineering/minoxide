use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::hitter_game::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateHitterGameDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateHitterGameDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateHitterGameDto> for ActiveModel {
    fn from(dto: CreateHitterGameDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateHitterGameDto> for ActiveModel")
    }
}

impl UpdateHitterGameDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateHitterGameDto::apply_to")
    }
}
