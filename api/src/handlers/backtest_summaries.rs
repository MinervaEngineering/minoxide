use minoa::postgres::backtest_summaries::{Entity as BacktestSummaryEntity, Model as BacktestSummaryModel, ActiveModel as BacktestSummaryActiveModel};
use crate::models::backtest_summaries::{CreateBacktestSummaryDto, UpdateBacktestSummaryDto};

crate::generate_crud_handlers!(
    BacktestSummaryEntity,
    BacktestSummaryModel,
    BacktestSummaryActiveModel,
    CreateBacktestSummaryDto,
    UpdateBacktestSummaryDto
);

pub use list as list_backtest_summaries;
pub use get as get_backtestsummaries;
pub use create as create_backtestsummaries;
pub use update as update_backtestsummaries;
pub use delete as delete_backtestsummaries;
