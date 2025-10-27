#!/bin/bash

# This script generates CRUD handlers and models for all entities

ENTITIES=(
    "teams:Team:teams"
    "venues:Venue:venue"
    "games:Game:game_id"
    "years:Year:years"
    "dates:Date:dates"
    "slates:Slate:slate"
    "dfs_players:DfsPlayer:dfs_players"
    "weather:Weather:weather"
    "projections:Projection:projections"
    "contest_list_parameters:ContestListParameter:contest_list_parameters"
    "backtests:Backtest:backtests"
    "backtest_strategies:BacktestStrategy:backtest_strategies"
    "contest_data:ContestData:contest_id"
    "backtest_files:BacktestFile:backtest_files"
    "backtest_lineups:BacktestLineup:backtest_lineups"
    "backtest_logs:BacktestLog:backtest_logs"
    "backtest_summaries:BacktestSummary:backtest_summaries"
    "backtest_results:BacktestResult:backtest_results"
    "backtest_contest_list:BacktestContestList:backtest_contest_list"
    "backtest_contest_summaries:BacktestContestSummary:backtest_contest_summaries"
    "backtests_complete:BacktestsComplete:backtests_complete"
    "hitter_projections:HitterProjection:hitter_projections"
    "pitcher_projections:PitcherProjection:pitcher_projections"
    "hitter_game:HitterGame:hitter_game"
    "pitcher_game:PitcherGame:pitcher_game"
    "live_slates:LiveSlate:live_slates"
    "dfn_live_hitters:DfnLiveHitter:dfn_live_hitters"
    "dfn_live_pitchers:DfnLivePitcher:dfn_live_pitchers"
    "mlb_live_lineups:MlbLiveLineup:mlb_live_lineups"
    "contests_pull:ContestsPull:contests_pull"
    "rpcs:Rpc:rpcs"
    "winning_charts:WinningChart:winning_charts"
)

echo "Generating CRUD files..."

for entity_info in "${ENTITIES[@]}"; do
    IFS=':' read -r module_name entity_name table_name <<< "$entity_info"
    
    # Create handler file
    cat > "api/src/handlers/${module_name}.rs" << EOF
use minoa::postgres::${module_name}::{Entity as ${entity_name}Entity, Model as ${entity_name}Model, ActiveModel as ${entity_name}ActiveModel};
use crate::models::${module_name}::{Create${entity_name}Dto, Update${entity_name}Dto};

crate::generate_crud_handlers!(
    ${entity_name}Entity,
    ${entity_name}Model,
    ${entity_name}ActiveModel,
    Create${entity_name}Dto,
    Update${entity_name}Dto
);

pub use list as list_${module_name};
pub use get as get_${module_name//_/};
pub use create as create_${module_name//_/};
pub use update as update_${module_name//_/};
pub use delete as delete_${module_name//_/};
EOF

    echo "Created handler: ${module_name}"
    
    # Create basic model file (will need manual adjustment for complex types)
    cat > "api/src/models/${module_name}.rs" << EOF
use serde::{Deserialize, Serialize};
use sea_orm::Set;
use minoa::postgres::${module_name}::ActiveModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct Create${entity_name}Dto {
    // TODO: Add fields based on entity definition
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Update${entity_name}Dto {
    // TODO: Add fields based on entity definition
}

impl From<Create${entity_name}Dto> for ActiveModel {
    fn from(dto: Create${entity_name}Dto) -> Self {
        // TODO: Implement conversion
        todo!("Implement From<Create${entity_name}Dto> for ActiveModel")
    }
}

impl Update${entity_name}Dto {
    pub fn apply_to(&self, _model: &mut ActiveModel) {
        // TODO: Implement update logic
        todo!("Implement Update${entity_name}Dto::apply_to")
    }
}
EOF

    echo "Created model: ${module_name}"
done

echo "Generation complete! Remember to:"
echo "1. Fill in the TODO sections in model files"
echo "2. Update Cargo.toml workspace to include 'api'"
echo "3. Test the API endpoints"
