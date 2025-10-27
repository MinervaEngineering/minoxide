use minoa::postgres::backtests_complete::{Entity as BacktestsCompleteEntity, Model as BacktestsCompleteModel, ActiveModel as BacktestsCompleteActiveModel};
use crate::models::backtests_complete::{CreateBacktestsCompleteDto, UpdateBacktestsCompleteDto};

crate::generate_crud_handlers!(
    BacktestsCompleteEntity,
    BacktestsCompleteModel,
    BacktestsCompleteActiveModel,
    CreateBacktestsCompleteDto,
    UpdateBacktestsCompleteDto
);

pub use list as list_backtests_complete;
pub use get as get_backtestscomplete;
pub use create as create_backtestscomplete;
pub use update as update_backtestscomplete;
pub use delete as delete_backtestscomplete;
