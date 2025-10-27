use minoa::postgres::players::ActiveModel;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePlayerDto {
    pub player_id: i32,
    pub player_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePlayerDto {
    pub player_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<CreatePlayerDto> for ActiveModel {
    fn from(dto: CreatePlayerDto) -> Self {
        ActiveModel {
            player_id: Set(dto.player_id),
            player_name: Set(dto.player_name),
            first_name: Set(dto.first_name),
            last_name: Set(dto.last_name),
        }
    }
}

impl UpdatePlayerDto {
    pub fn apply_to(&self, model: &mut ActiveModel) {
        if let Some(ref name) = self.player_name {
            model.player_name = Set(Some(name.clone()));
        }
        if let Some(ref first) = self.first_name {
            model.first_name = Set(Some(first.clone()));
        }
        if let Some(ref last) = self.last_name {
            model.last_name = Set(Some(last.clone()));
        }
    }
}
