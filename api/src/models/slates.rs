use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::slates::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSlateDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSlateDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateSlateDto> for ActiveModel {
    fn from(dto: CreateSlateDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateSlateDto> for ActiveModel")
    }
}

impl UpdateSlateDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateSlateDto::apply_to")
    }
}
