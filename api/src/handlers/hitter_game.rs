use minoa::postgres::hitter_game::{Entity as HitterGameEntity, Model as HitterGameModel, ActiveModel as HitterGameActiveModel};
use crate::models::hitter_game::{CreateHitterGameDto, UpdateHitterGameDto};

crate::generate_crud_handlers!(
    HitterGameEntity,
    HitterGameModel,
    HitterGameActiveModel,
    CreateHitterGameDto,
    UpdateHitterGameDto
);

pub use list as list_hitter_game;
pub use get as get_hittergame;
pub use create as create_hittergame;
pub use update as update_hittergame;
pub use delete as delete_hittergame;
