use minoa::postgres::contests_pull::{Entity as ContestsPullEntity, Model as ContestsPullModel, ActiveModel as ContestsPullActiveModel};
use crate::models::contests_pull::{CreateContestsPullDto, UpdateContestsPullDto};

crate::generate_crud_handlers!(
    ContestsPullEntity,
    ContestsPullModel,
    ContestsPullActiveModel,
    CreateContestsPullDto,
    UpdateContestsPullDto
);

pub use list as list_contests_pull;
pub use get as get_contestspull;
pub use create as create_contestspull;
pub use update as update_contestspull;
pub use delete as delete_contestspull;
