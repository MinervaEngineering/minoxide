use minoa::postgres::backtest_logs::{Entity as BacktestLogEntity, Model as BacktestLogModel, ActiveModel as BacktestLogActiveModel};
use crate::models::backtest_logs::{CreateBacktestLogDto, UpdateBacktestLogDto};

crate::generate_crud_handlers!(
    BacktestLogEntity,
    BacktestLogModel,
    BacktestLogActiveModel,
    CreateBacktestLogDto,
    UpdateBacktestLogDto
);

pub use list as list_backtest_logs;
pub use get as get_backtestlogs;
pub use create as create_backtestlogs;
pub use update as update_backtestlogs;
pub use delete as delete_backtestlogs;
