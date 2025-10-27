use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtest_files::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestFileDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestFileDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestFileDto> for ActiveModel {
    fn from(dto: CreateBacktestFileDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestFileDto> for ActiveModel")
    }
}

impl UpdateBacktestFileDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestFileDto::apply_to")
    }
}
