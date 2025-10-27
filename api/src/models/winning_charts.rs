use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::winning_charts::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWinningChartDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWinningChartDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateWinningChartDto> for ActiveModel {
    fn from(dto: CreateWinningChartDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateWinningChartDto> for ActiveModel")
    }
}

impl UpdateWinningChartDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateWinningChartDto::apply_to")
    }
}
