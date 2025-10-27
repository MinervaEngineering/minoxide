use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::contests_pull::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContestsPullDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateContestsPullDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateContestsPullDto> for ActiveModel {
    fn from(dto: CreateContestsPullDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateContestsPullDto> for ActiveModel")
    }
}

impl UpdateContestsPullDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateContestsPullDto::apply_to")
    }
}
