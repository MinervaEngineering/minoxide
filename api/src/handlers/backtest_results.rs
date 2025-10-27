use minoa::postgres::backtest_results::{Entity as BacktestResultEntity, Model as BacktestResultModel, ActiveModel as BacktestResultActiveModel};
use crate::models::backtest_results::{CreateBacktestResultDto, UpdateBacktestResultDto};

crate::generate_crud_handlers!(
    BacktestResultEntity,
    BacktestResultModel,
    BacktestResultActiveModel,
    CreateBacktestResultDto,
    UpdateBacktestResultDto
);

pub use list as list_backtest_results;
pub use get as get_backtestresults;
pub use create as create_backtestresults;
pub use update as update_backtestresults;
pub use delete as delete_backtestresults;
