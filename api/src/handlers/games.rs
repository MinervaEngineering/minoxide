use minoa::postgres::games::{Entity as GameEntity, Model as GameModel, ActiveModel as GameActiveModel};
use crate::models::games::{CreateGameDto, UpdateGameDto};

crate::generate_crud_handlers!(
    GameEntity,
    GameModel,
    GameActiveModel,
    CreateGameDto,
    UpdateGameDto
);

pub use list as list_games;
pub use get as get_games;
pub use create as create_games;
pub use update as update_games;
pub use delete as delete_games;
