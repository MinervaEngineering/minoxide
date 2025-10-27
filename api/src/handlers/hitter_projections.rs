use minoa::postgres::hitter_projections::{Entity as HitterProjectionEntity, Model as HitterProjectionModel, ActiveModel as HitterProjectionActiveModel};
use crate::models::hitter_projections::{CreateHitterProjectionDto, UpdateHitterProjectionDto};

crate::generate_crud_handlers!(
    HitterProjectionEntity,
    HitterProjectionModel,
    HitterProjectionActiveModel,
    CreateHitterProjectionDto,
    UpdateHitterProjectionDto
);

pub use list as list_hitter_projections;
pub use get as get_hitterprojections;
pub use create as create_hitterprojections;
pub use update as update_hitterprojections;
pub use delete as delete_hitterprojections;
