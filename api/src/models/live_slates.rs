use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::live_slates::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLiveSlateDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateLiveSlateDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateLiveSlateDto> for ActiveModel {
    fn from(dto: CreateLiveSlateDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateLiveSlateDto> for ActiveModel")
    }
}

impl UpdateLiveSlateDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateLiveSlateDto::apply_to")
    }
}
