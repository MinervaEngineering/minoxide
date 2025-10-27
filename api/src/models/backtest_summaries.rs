use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtest_summaries::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestSummaryDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestSummaryDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestSummaryDto> for ActiveModel {
    fn from(dto: CreateBacktestSummaryDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestSummaryDto> for ActiveModel")
    }
}

impl UpdateBacktestSummaryDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestSummaryDto::apply_to")
    }
}
