use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtests_complete::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestsCompleteDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestsCompleteDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestsCompleteDto> for ActiveModel {
    fn from(dto: CreateBacktestsCompleteDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestsCompleteDto> for ActiveModel")
    }
}

impl UpdateBacktestsCompleteDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestsCompleteDto::apply_to")
    }
}
