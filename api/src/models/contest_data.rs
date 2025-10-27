use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::contest_data::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContestDataDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateContestDataDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateContestDataDto> for ActiveModel {
    fn from(dto: CreateContestDataDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateContestDataDto> for ActiveModel")
    }
}

impl UpdateContestDataDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateContestDataDto::apply_to")
    }
}
