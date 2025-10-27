use minoa::postgres::backtest_contest_summaries::{Entity as BacktestContestSummaryEntity, Model as BacktestContestSummaryModel, ActiveModel as BacktestContestSummaryActiveModel};
use crate::models::backtest_contest_summaries::{CreateBacktestContestSummaryDto, UpdateBacktestContestSummaryDto};

crate::generate_crud_handlers!(
    BacktestContestSummaryEntity,
    BacktestContestSummaryModel,
    BacktestContestSummaryActiveModel,
    CreateBacktestContestSummaryDto,
    UpdateBacktestContestSummaryDto
);

pub use list as list_backtest_contest_summaries;
pub use get as get_backtestcontestsummaries;
pub use create as create_backtestcontestsummaries;
pub use update as update_backtestcontestsummaries;
pub use delete as delete_backtestcontestsummaries;
