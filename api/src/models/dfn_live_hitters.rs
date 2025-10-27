use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::dfn_live_hitters::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDfnLiveHitterDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDfnLiveHitterDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateDfnLiveHitterDto> for ActiveModel {
    fn from(dto: CreateDfnLiveHitterDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateDfnLiveHitterDto> for ActiveModel")
    }
}

impl UpdateDfnLiveHitterDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateDfnLiveHitterDto::apply_to")
    }
}
