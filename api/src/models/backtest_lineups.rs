use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::backtest_lineups::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBacktestLineupDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBacktestLineupDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateBacktestLineupDto> for ActiveModel {
    fn from(dto: CreateBacktestLineupDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateBacktestLineupDto> for ActiveModel")
    }
}

impl UpdateBacktestLineupDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateBacktestLineupDto::apply_to")
    }
}
