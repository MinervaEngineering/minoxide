use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::contest_list_parameters::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContestListParameterDto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateContestListParameterDto {
    // TODO: Add fields based on entity definition
}

impl From<CreateContestListParameterDto> for ActiveModel {
    fn from(dto: CreateContestListParameterDto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<CreateContestListParameterDto> for ActiveModel")
    }
}

impl UpdateContestListParameterDto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement UpdateContestListParameterDto::apply_to")
    }
}
