use minoa::postgres::dfn_live_hitters::{Entity as DfnLiveHitterEntity, Model as DfnLiveHitterModel, ActiveModel as DfnLiveHitterActiveModel};
use crate::models::dfn_live_hitters::{CreateDfnLiveHitterDto, UpdateDfnLiveHitterDto};

crate::generate_crud_handlers!(
    DfnLiveHitterEntity,
    DfnLiveHitterModel,
    DfnLiveHitterActiveModel,
    CreateDfnLiveHitterDto,
    UpdateDfnLiveHitterDto
);

pub use list as list_dfn_live_hitters;
pub use get as get_dfnlivehitters;
pub use create as create_dfnlivehitters;
pub use update as update_dfnlivehitters;
pub use delete as delete_dfnlivehitters;
