use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::hitter_projections::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateHitterProjectionDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateHitterProjectionDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateHitterProjectionDto> for ActiveModel {
    fn from(dto: CreateHitterProjectionDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateHitterProjectionDto> for ActiveModel")
    }
}

impl UpdateHitterProjectionDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateHitterProjectionDto::apply_to")
    }
}
