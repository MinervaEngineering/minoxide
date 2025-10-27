use minoa::postgres::backtest_files::{Entity as BacktestFileEntity, Model as BacktestFileModel, ActiveModel as BacktestFileActiveModel};
use crate::models::backtest_files::{CreateBacktestFileDto, UpdateBacktestFileDto};

crate::generate_crud_handlers!(
    BacktestFileEntity,
    BacktestFileModel,
    BacktestFileActiveModel,
    CreateBacktestFileDto,
    UpdateBacktestFileDto
);

pub use list as list_backtest_files;
pub use get as get_backtestfiles;
pub use create as create_backtestfiles;
pub use update as update_backtestfiles;
pub use delete as delete_backtestfiles;
