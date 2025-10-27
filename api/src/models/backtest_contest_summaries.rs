use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtest_contest_summaries::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestContestSummaryDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestContestSummaryDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestContestSummaryDto> for ActiveModel {
    fn from(dto: CreateBacktestContestSummaryDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestContestSummaryDto> for ActiveModel")
    }
}

impl UpdateBacktestContestSummaryDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestContestSummaryDto::apply_to")
    }
}
