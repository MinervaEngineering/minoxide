use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::venues::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateVenueDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateVenueDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateVenueDto> for ActiveModel {
    fn from(dto: CreateVenueDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateVenueDto> for ActiveModel")
    }
}

impl UpdateVenueDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateVenueDto::apply_to")
    }
}
