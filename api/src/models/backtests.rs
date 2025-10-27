use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtests::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestDto> for ActiveModel {
    fn from(dto: CreateBacktestDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestDto> for ActiveModel")
    }
}

impl UpdateBacktestDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestDto::apply_to")
    }
}
