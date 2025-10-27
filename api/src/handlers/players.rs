use crate::models::players::{CreatePlayerDto, UpdatePlayerDto};
use minoa::postgres::players::{
    ActiveModel as PlayerActiveModel, Entity as PlayerEntity, Model as PlayerModel,
};

// Generate standard CRUD handlers using the macro
crate::generate_crud_handlers!(
    PlayerEntity,
    PlayerModel,
    PlayerActiveModel,
    CreatePlayerDto,
    UpdatePlayerDto
);

// Rename exports to match router expectations
pub use create as create_player;
pub use delete as delete_player;
pub use get as get_player;
pub use list as list_players;
pub use update as update_player;
