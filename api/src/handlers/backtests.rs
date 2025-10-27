use minoa::postgres::backtests::{Entity as BacktestEntity, Model as BacktestModel, ActiveModel as BacktestActiveModel};
use crate::models::backtests::{CreateBacktestDto, UpdateBacktestDto};

crate::generate_crud_handlers!(
    BacktestEntity,
    BacktestModel,
    BacktestActiveModel,
    CreateBacktestDto,
    UpdateBacktestDto
);

pub use list as list_backtests;
pub use get as get_backtests;
pub use create as create_backtests;
pub use update as update_backtests;
pub use delete as delete_backtests;
