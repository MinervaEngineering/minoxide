use minoa::postgres::dfs_players::{Entity as DfsPlayerEntity, Model as DfsPlayerModel, ActiveModel as DfsPlayerActiveModel};
use crate::models::dfs_players::{CreateDfsPlayerDto, UpdateDfsPlayerDto};

crate::generate_crud_handlers!(
    DfsPlayerEntity,
    DfsPlayerModel,
    DfsPlayerActiveModel,
    CreateDfsPlayerDto,
    UpdateDfsPlayerDto
);

pub use list as list_dfs_players;
pub use get as get_dfsplayers;
pub use create as create_dfsplayers;
pub use update as update_dfsplayers;
pub use delete as delete_dfsplayers;
