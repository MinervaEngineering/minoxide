use minoa::postgres::backtest_strategies::{Entity as BacktestStrategyEntity, Model as BacktestStrategyModel, ActiveModel as BacktestStrategyActiveModel};
use crate::models::backtest_strategies::{CreateBacktestStrategyDto, UpdateBacktestStrategyDto};

crate::generate_crud_handlers!(
    BacktestStrategyEntity,
    BacktestStrategyModel,
    BacktestStrategyActiveModel,
    CreateBacktestStrategyDto,
    UpdateBacktestStrategyDto
);

pub use list as list_backtest_strategies;
pub use get as get_backteststrategies;
pub use create as create_backteststrategies;
pub use update as update_backteststrategies;
pub use delete as delete_backteststrategies;
