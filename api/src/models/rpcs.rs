use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::rpcs::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRpcDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRpcDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateRpcDto> for ActiveModel {
    fn from(dto: CreateRpcDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateRpcDto> for ActiveModel")
    }
}

impl UpdateRpcDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateRpcDto::apply_to")
    }
}
