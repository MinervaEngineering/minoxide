use minoa::postgres::pitcher_projections::{Entity as PitcherProjectionEntity, Model as PitcherProjectionModel, ActiveModel as PitcherProjectionActiveModel};
use crate::models::pitcher_projections::{CreatePitcherProjectionDto, UpdatePitcherProjectionDto};

crate::generate_crud_handlers!(
    PitcherProjectionEntity,
    PitcherProjectionModel,
    PitcherProjectionActiveModel,
    CreatePitcherProjectionDto,
    UpdatePitcherProjectionDto
);

pub use list as list_pitcher_projections;
pub use get as get_pitcherprojections;
pub use create as create_pitcherprojections;
pub use update as update_pitcherprojections;
pub use delete as delete_pitcherprojections;
