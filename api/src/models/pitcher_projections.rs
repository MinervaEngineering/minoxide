use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::pitcher_projections::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePitcherProjectionDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePitcherProjectionDto {
    // TODO: Add fields based on entity definition
}

impl From<CreatePitcherProjectionDto> for ActiveModel {
    fn from(dto: CreatePitcherProjectionDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreatePitcherProjectionDto> for ActiveModel")
    }
}

impl UpdatePitcherProjectionDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdatePitcherProjectionDto::apply_to")
    }
}
