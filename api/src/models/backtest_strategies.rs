use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtest_strategies::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestStrategyDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestStrategyDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestStrategyDto> for ActiveModel {
    fn from(dto: CreateBacktestStrategyDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestStrategyDto> for ActiveModel")
    }
}

impl UpdateBacktestStrategyDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestStrategyDto::apply_to")
    }
}
