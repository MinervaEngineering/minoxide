use minoa::postgres::teams::{Entity as TeamEntity, Model as TeamModel, ActiveModel as TeamActiveModel};
use crate::models::teams::{CreateTeamDto, UpdateTeamDto};

crate::generate_crud_handlers!(
    TeamEntity,
    TeamModel,
    TeamActiveModel,
    CreateTeamDto,
    UpdateTeamDto
);

pub use list as list_teams;
pub use get as get_teams;
pub use create as create_teams;
pub use update as update_teams;
pub use delete as delete_teams;
