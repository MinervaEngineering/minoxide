use minoa::postgres::mlb_live_lineups::{Entity as MlbLiveLineupEntity, Model as MlbLiveLineupModel, ActiveModel as MlbLiveLineupActiveModel};
use crate::models::mlb_live_lineups::{CreateMlbLiveLineupDto, UpdateMlbLiveLineupDto};

crate::generate_crud_handlers!(
    MlbLiveLineupEntity,
    MlbLiveLineupModel,
    MlbLiveLineupActiveModel,
    CreateMlbLiveLineupDto,
    UpdateMlbLiveLineupDto
);

pub use list as list_mlb_live_lineups;
pub use get as get_mlblivelineups;
pub use create as create_mlblivelineups;
pub use update as update_mlblivelineups;
pub use delete as delete_mlblivelineups;
