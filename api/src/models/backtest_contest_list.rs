use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtest_contest_list::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestContestListDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestContestListDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestContestListDto> for ActiveModel {
    fn from(dto: CreateBacktestContestListDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestContestListDto> for ActiveModel")
    }
}

impl UpdateBacktestContestListDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestContestListDto::apply_to")
    }
}
