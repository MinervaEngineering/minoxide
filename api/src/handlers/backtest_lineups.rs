use minoa::postgres::backtest_lineups::{Entity as BacktestLineupEntity, Model as BacktestLineupModel, ActiveModel as BacktestLineupActiveModel};
use crate::models::backtest_lineups::{CreateBacktestLineupDto, UpdateBacktestLineupDto};

crate::generate_crud_handlers!(
    BacktestLineupEntity,
    BacktestLineupModel,
    BacktestLineupActiveModel,
    CreateBacktestLineupDto,
    UpdateBacktestLineupDto
);

pub use list as list_backtest_lineups;
pub use get as get_backtestlineups;
pub use create as create_backtestlineups;
pub use update as update_backtestlineups;
pub use delete as delete_backtestlineups;
