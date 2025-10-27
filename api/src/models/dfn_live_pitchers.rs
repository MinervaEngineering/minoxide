use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::dfn_live_pitchers::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDfnLivePitcherDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDfnLivePitcherDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateDfnLivePitcherDto> for ActiveModel {
    fn from(dto: CreateDfnLivePitcherDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateDfnLivePitcherDto> for ActiveModel")
    }
}

impl UpdateDfnLivePitcherDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateDfnLivePitcherDto::apply_to")
    }
}
