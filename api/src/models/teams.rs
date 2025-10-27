use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::teams::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTeamDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTeamDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateTeamDto> for ActiveModel {
    fn from(dto: CreateTeamDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateTeamDto> for ActiveModel")
    }
}

impl UpdateTeamDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateTeamDto::apply_to")
    }
}
