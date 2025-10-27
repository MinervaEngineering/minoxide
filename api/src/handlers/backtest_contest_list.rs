use minoa::postgres::backtest_contest_list::{Entity as BacktestContestListEntity, Model as BacktestContestListModel, ActiveModel as BacktestContestListActiveModel};
use crate::models::backtest_contest_list::{CreateBacktestContestListDto, UpdateBacktestContestListDto};

crate::generate_crud_handlers!(
    BacktestContestListEntity,
    BacktestContestListModel,
    BacktestContestListActiveModel,
    CreateBacktestContestListDto,
    UpdateBacktestContestListDto
);

pub use list as list_backtest_contest_list;
pub use get as get_backtestcontestlist;
pub use create as create_backtestcontestlist;
pub use update as update_backtestcontestlist;
pub use delete as delete_backtestcontestlist;
