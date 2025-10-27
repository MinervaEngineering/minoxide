use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::projections::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectionDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProjectionDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateProjectionDto> for ActiveModel {
    fn from(dto: CreateProjectionDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateProjectionDto> for ActiveModel")
    }
}

impl UpdateProjectionDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateProjectionDto::apply_to")
    }
}
