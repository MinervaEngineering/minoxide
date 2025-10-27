use minoa::postgres::pitcher_game::{Entity as PitcherGameEntity, Model as PitcherGameModel, ActiveModel as PitcherGameActiveModel};
use crate::models::pitcher_game::{CreatePitcherGameDto, UpdatePitcherGameDto};

crate::generate_crud_handlers!(
    PitcherGameEntity,
    PitcherGameModel,
    PitcherGameActiveModel,
    CreatePitcherGameDto,
    UpdatePitcherGameDto
);

pub use list as list_pitcher_game;
pub use get as get_pitchergame;
pub use create as create_pitchergame;
pub use update as update_pitchergame;
pub use delete as delete_pitchergame;
