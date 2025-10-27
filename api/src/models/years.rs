use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::years::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateYearDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateYearDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateYearDto> for ActiveModel {
    fn from(dto: CreateYearDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateYearDto> for ActiveModel")
    }
}

impl UpdateYearDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateYearDto::apply_to")
    }
}
