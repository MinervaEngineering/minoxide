use minoa::postgres::rpcs::{Entity as RpcEntity, Model as RpcModel, ActiveModel as RpcActiveModel};
use crate::models::rpcs::{CreateRpcDto, UpdateRpcDto};

crate::generate_crud_handlers!(
    RpcEntity,
    RpcModel,
    RpcActiveModel,
    CreateRpcDto,
    UpdateRpcDto
);

pub use list as list_rpcs;
pub use get as get_rpcs;
pub use create as create_rpcs;
pub use update as update_rpcs;
pub use delete as delete_rpcs;
