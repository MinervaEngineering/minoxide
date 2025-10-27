use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::dfs_players::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDfsPlayerDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDfsPlayerDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateDfsPlayerDto> for ActiveModel {
    fn from(dto: CreateDfsPlayerDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateDfsPlayerDto> for ActiveModel")
    }
}

impl UpdateDfsPlayerDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateDfsPlayerDto::apply_to")
    }
}
