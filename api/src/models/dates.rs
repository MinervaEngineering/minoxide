use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::dates::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDateDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDateDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateDateDto> for ActiveModel {
    fn from(dto: CreateDateDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateDateDto> for ActiveModel")
    }
}

impl UpdateDateDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateDateDto::apply_to")
    }
}
