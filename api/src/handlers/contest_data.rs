use minoa::postgres::contest_data::{Entity as ContestDataEntity, Model as ContestDataModel, ActiveModel as ContestDataActiveModel};
use crate::models::contest_data::{CreateContestDataDto, UpdateContestDataDto};

crate::generate_crud_handlers!(
    ContestDataEntity,
    ContestDataModel,
    ContestDataActiveModel,
    CreateContestDataDto,
    UpdateContestDataDto
);

pub use list as list_contest_data;
pub use get as get_contestdata;
pub use create as create_contestdata;
pub use update as update_contestdata;
pub use delete as delete_contestdata;
