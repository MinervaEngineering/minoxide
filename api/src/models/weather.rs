use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::weather::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWeatherDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWeatherDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateWeatherDto> for ActiveModel {
    fn from(dto: CreateWeatherDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateWeatherDto> for ActiveModel")
    }
}

impl UpdateWeatherDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateWeatherDto::apply_to")
    }
}
