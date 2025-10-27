use minoa::postgres::dfn_live_pitchers::{Entity as DfnLivePitcherEntity, Model as DfnLivePitcherModel, ActiveModel as DfnLivePitcherActiveModel};
use crate::models::dfn_live_pitchers::{CreateDfnLivePitcherDto, UpdateDfnLivePitcherDto};

crate::generate_crud_handlers!(
    DfnLivePitcherEntity,
    DfnLivePitcherModel,
    DfnLivePitcherActiveModel,
    CreateDfnLivePitcherDto,
    UpdateDfnLivePitcherDto
);

pub use list as list_dfn_live_pitchers;
pub use get as get_dfnlivepitchers;
pub use create as create_dfnlivepitchers;
pub use update as update_dfnlivepitchers;
pub use delete as delete_dfnlivepitchers;
