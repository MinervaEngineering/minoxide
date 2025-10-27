use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::mlb_live_lineups::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMlbLiveLineupDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateMlbLiveLineupDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateMlbLiveLineupDto> for ActiveModel {
    fn from(dto: CreateMlbLiveLineupDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateMlbLiveLineupDto> for ActiveModel")
    }
}

impl UpdateMlbLiveLineupDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateMlbLiveLineupDto::apply_to")
    }
}
