use minoa::postgres::winning_charts::{Entity as WinningChartEntity, Model as WinningChartModel, ActiveModel as WinningChartActiveModel};
use crate::models::winning_charts::{CreateWinningChartDto, UpdateWinningChartDto};

crate::generate_crud_handlers!(
    WinningChartEntity,
    WinningChartModel,
    WinningChartActiveModel,
    CreateWinningChartDto,
    UpdateWinningChartDto
);

pub use list as list_winning_charts;
pub use get as get_winningcharts;
pub use create as create_winningcharts;
pub use update as update_winningcharts;
pub use delete as delete_winningcharts;
