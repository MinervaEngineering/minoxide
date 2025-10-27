use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtest_results::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestResultDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestResultDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestResultDto> for ActiveModel {
    fn from(dto: CreateBacktestResultDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestResultDto> for ActiveModel")
    }
}

impl UpdateBacktestResultDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestResultDto::apply_to")
    }
}
