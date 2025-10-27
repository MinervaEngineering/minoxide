mod error;
mod handlers;
mod models;
mod routes;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "minoa_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to database
    tracing::info!("Connecting to database...");
    let db = Database::connect(&database_url).await?;
    tracing::info!("Database connected successfully");

    let state = AppState { db };

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        // Players routes
        .route("/api/players", get(handlers::players::list_players))
        .route("/api/players", post(handlers::players::create_player))
        .route("/api/players/:id", get(handlers::players::get_player))
        .route("/api/players/:id", put(handlers::players::update_player))
        .route("/api/players/:id", delete(handlers::players::delete_player))
        // Teams routes
        .route("/api/teams", get(handlers::teams::list_teams))
        .route("/api/teams", post(handlers::teams::create_team))
        .route("/api/teams/:id", get(handlers::teams::get_team))
        .route("/api/teams/:id", put(handlers::teams::update_team))
        .route("/api/teams/:id", delete(handlers::teams::delete_team))
        // Venues routes
        .route("/api/venues", get(handlers::venues::list_venues))
        .route("/api/venues", post(handlers::venues::create_venue))
        .route("/api/venues/:id", get(handlers::venues::get_venue))
        .route("/api/venues/:id", put(handlers::venues::update_venue))
        .route("/api/venues/:id", delete(handlers::venues::delete_venue))
        // Games routes
        .route("/api/games", get(handlers::games::list_games))
        .route("/api/games", post(handlers::games::create_game))
        .route("/api/games/:id", get(handlers::games::get_game))
        .route("/api/games/:id", put(handlers::games::update_game))
        .route("/api/games/:id", delete(handlers::games::delete_game))
        // Years routes
        .route("/api/years", get(handlers::years::list_years))
        .route("/api/years", post(handlers::years::create_year))
        .route("/api/years/:id", get(handlers::years::get_year))
        .route("/api/years/:id", put(handlers::years::update_year))
        .route("/api/years/:id", delete(handlers::years::delete_year))
        // Dates routes
        .route("/api/dates", get(handlers::dates::list_dates))
        .route("/api/dates", post(handlers::dates::create_date))
        .route("/api/dates/:id", get(handlers::dates::get_date))
        .route("/api/dates/:id", put(handlers::dates::update_date))
        .route("/api/dates/:id", delete(handlers::dates::delete_date))
        // Slates routes
        .route("/api/slates", get(handlers::slates::list_slates))
        .route("/api/slates", post(handlers::slates::create_slate))
        .route("/api/slates/:id", get(handlers::slates::get_slate))
        .route("/api/slates/:id", put(handlers::slates::update_slate))
        .route("/api/slates/:id", delete(handlers::slates::delete_slate))
        // DFS Players routes
        .route(
            "/api/dfs-players",
            get(handlers::dfs_players::list_dfs_players),
        )
        .route(
            "/api/dfs-players",
            post(handlers::dfs_players::create_dfs_player),
        )
        .route(
            "/api/dfs-players/:id",
            get(handlers::dfs_players::get_dfs_player),
        )
        .route(
            "/api/dfs-players/:id",
            put(handlers::dfs_players::update_dfs_player),
        )
        .route(
            "/api/dfs-players/:id",
            delete(handlers::dfs_players::delete_dfs_player),
        )
        // Weather routes
        .route("/api/weather", get(handlers::weather::list_weather))
        .route("/api/weather", post(handlers::weather::create_weather))
        .route("/api/weather/:id", get(handlers::weather::get_weather))
        .route("/api/weather/:id", put(handlers::weather::update_weather))
        .route(
            "/api/weather/:id",
            delete(handlers::weather::delete_weather),
        )
        // Projections routes
        .route(
            "/api/projections",
            get(handlers::projections::list_projections),
        )
        .route(
            "/api/projections",
            post(handlers::projections::create_projection),
        )
        .route(
            "/api/projections/:id",
            get(handlers::projections::get_projection),
        )
        .route(
            "/api/projections/:id",
            put(handlers::projections::update_projection),
        )
        .route(
            "/api/projections/:id",
            delete(handlers::projections::delete_projection),
        )
        // Contest List Parameters routes
        .route(
            "/api/contest-list-parameters",
            get(handlers::contest_list_parameters::list_contest_list_parameters),
        )
        .route(
            "/api/contest-list-parameters",
            post(handlers::contest_list_parameters::create_contest_list_parameter),
        )
        .route(
            "/api/contest-list-parameters/:id",
            get(handlers::contest_list_parameters::get_contest_list_parameter),
        )
        .route(
            "/api/contest-list-parameters/:id",
            put(handlers::contest_list_parameters::update_contest_list_parameter),
        )
        .route(
            "/api/contest-list-parameters/:id",
            delete(handlers::contest_list_parameters::delete_contest_list_parameter),
        )
        // Backtests routes
        .route("/api/backtests", get(handlers::backtests::list_backtests))
        .route("/api/backtests", post(handlers::backtests::create_backtest))
        .route("/api/backtests/:id", get(handlers::backtests::get_backtest))
        .route(
            "/api/backtests/:id",
            put(handlers::backtests::update_backtest),
        )
        .route(
            "/api/backtests/:id",
            delete(handlers::backtests::delete_backtest),
        )
        // Backtest Strategies routes
        .route(
            "/api/backtest-strategies",
            get(handlers::backtest_strategies::list_backtest_strategies),
        )
        .route(
            "/api/backtest-strategies",
            post(handlers::backtest_strategies::create_backtest_strategy),
        )
        .route(
            "/api/backtest-strategies/:id",
            get(handlers::backtest_strategies::get_backtest_strategy),
        )
        .route(
            "/api/backtest-strategies/:id",
            put(handlers::backtest_strategies::update_backtest_strategy),
        )
        .route(
            "/api/backtest-strategies/:id",
            delete(handlers::backtest_strategies::delete_backtest_strategy),
        )
        // Contest Data routes
        .route(
            "/api/contest-data",
            get(handlers::contest_data::list_contest_data),
        )
        .route(
            "/api/contest-data",
            post(handlers::contest_data::create_contest_data),
        )
        .route(
            "/api/contest-data/:id",
            get(handlers::contest_data::get_contest_data),
        )
        .route(
            "/api/contest-data/:id",
            put(handlers::contest_data::update_contest_data),
        )
        .route(
            "/api/contest-data/:id",
            delete(handlers::contest_data::delete_contest_data),
        )
        // Backtest Files routes
        .route(
            "/api/backtest-files",
            get(handlers::backtest_files::list_backtest_files),
        )
        .route(
            "/api/backtest-files",
            post(handlers::backtest_files::create_backtest_file),
        )
        .route(
            "/api/backtest-files/:id",
            get(handlers::backtest_files::get_backtest_file),
        )
        .route(
            "/api/backtest-files/:id",
            put(handlers::backtest_files::update_backtest_file),
        )
        .route(
            "/api/backtest-files/:id",
            delete(handlers::backtest_files::delete_backtest_file),
        )
        // Backtest Lineups routes
        .route(
            "/api/backtest-lineups",
            get(handlers::backtest_lineups::list_backtest_lineups),
        )
        .route(
            "/api/backtest-lineups",
            post(handlers::backtest_lineups::create_backtest_lineup),
        )
        .route(
            "/api/backtest-lineups/:id",
            get(handlers::backtest_lineups::get_backtest_lineup),
        )
        .route(
            "/api/backtest-lineups/:id",
            put(handlers::backtest_lineups::update_backtest_lineup),
        )
        .route(
            "/api/backtest-lineups/:id",
            delete(handlers::backtest_lineups::delete_backtest_lineup),
        )
        // Backtest Logs routes
        .route(
            "/api/backtest-logs",
            get(handlers::backtest_logs::list_backtest_logs),
        )
        .route(
            "/api/backtest-logs",
            post(handlers::backtest_logs::create_backtest_log),
        )
        .route(
            "/api/backtest-logs/:id",
            get(handlers::backtest_logs::get_backtest_log),
        )
        .route(
            "/api/backtest-logs/:id",
            put(handlers::backtest_logs::update_backtest_log),
        )
        .route(
            "/api/backtest-logs/:id",
            delete(handlers::backtest_logs::delete_backtest_log),
        )
        // Backtest Summaries routes
        .route(
            "/api/backtest-summaries",
            get(handlers::backtest_summaries::list_backtest_summaries),
        )
        .route(
            "/api/backtest-summaries",
            post(handlers::backtest_summaries::create_backtest_summary),
        )
        .route(
            "/api/backtest-summaries/:id",
            get(handlers::backtest_summaries::get_backtest_summary),
        )
        .route(
            "/api/backtest-summaries/:id",
            put(handlers::backtest_summaries::update_backtest_summary),
        )
        .route(
            "/api/backtest-summaries/:id",
            delete(handlers::backtest_summaries::delete_backtest_summary),
        )
        // Backtest Results routes
        .route(
            "/api/backtest-results",
            get(handlers::backtest_results::list_backtest_results),
        )
        .route(
            "/api/backtest-results",
            post(handlers::backtest_results::create_backtest_result),
        )
        .route(
            "/api/backtest-results/:id",
            get(handlers::backtest_results::get_backtest_result),
        )
        .route(
            "/api/backtest-results/:id",
            put(handlers::backtest_results::update_backtest_result),
        )
        .route(
            "/api/backtest-results/:id",
            delete(handlers::backtest_results::delete_backtest_result),
        )
        // Backtest Contest List routes
        .route(
            "/api/backtest-contest-list",
            get(handlers::backtest_contest_list::list_backtest_contest_list),
        )
        .route(
            "/api/backtest-contest-list",
            post(handlers::backtest_contest_list::create_backtest_contest_list),
        )
        .route(
            "/api/backtest-contest-list/:id",
            get(handlers::backtest_contest_list::get_backtest_contest_list),
        )
        .route(
            "/api/backtest-contest-list/:id",
            put(handlers::backtest_contest_list::update_backtest_contest_list),
        )
        .route(
            "/api/backtest-contest-list/:id",
            delete(handlers::backtest_contest_list::delete_backtest_contest_list),
        )
        // Backtest Contest Summaries routes
        .route(
            "/api/backtest-contest-summaries",
            get(handlers::backtest_contest_summaries::list_backtest_contest_summaries),
        )
        .route(
            "/api/backtest-contest-summaries",
            post(handlers::backtest_contest_summaries::create_backtest_contest_summary),
        )
        .route(
            "/api/backtest-contest-summaries/:id",
            get(handlers::backtest_contest_summaries::get_backtest_contest_summary),
        )
        .route(
            "/api/backtest-contest-summaries/:id",
            put(handlers::backtest_contest_summaries::update_backtest_contest_summary),
        )
        .route(
            "/api/backtest-contest-summaries/:id",
            delete(handlers::backtest_contest_summaries::delete_backtest_contest_summary),
        )
        // Backtests Complete routes
        .route(
            "/api/backtests-complete",
            get(handlers::backtests_complete::list_backtests_complete),
        )
        .route(
            "/api/backtests-complete",
            post(handlers::backtests_complete::create_backtests_complete),
        )
        .route(
            "/api/backtests-complete/:id",
            get(handlers::backtests_complete::get_backtests_complete),
        )
        .route(
            "/api/backtests-complete/:id",
            put(handlers::backtests_complete::update_backtests_complete),
        )
        .route(
            "/api/backtests-complete/:id",
            delete(handlers::backtests_complete::delete_backtests_complete),
        )
        // Hitter Projections routes
        .route(
            "/api/hitter-projections",
            get(handlers::hitter_projections::list_hitter_projections),
        )
        .route(
            "/api/hitter-projections",
            post(handlers::hitter_projections::create_hitter_projection),
        )
        .route(
            "/api/hitter-projections/:id",
            get(handlers::hitter_projections::get_hitter_projection),
        )
        .route(
            "/api/hitter-projections/:id",
            put(handlers::hitter_projections::update_hitter_projection),
        )
        .route(
            "/api/hitter-projections/:id",
            delete(handlers::hitter_projections::delete_hitter_projection),
        )
        // Pitcher Projections routes
        .route(
            "/api/pitcher-projections",
            get(handlers::pitcher_projections::list_pitcher_projections),
        )
        .route(
            "/api/pitcher-projections",
            post(handlers::pitcher_projections::create_pitcher_projection),
        )
        .route(
            "/api/pitcher-projections/:id",
            get(handlers::pitcher_projections::get_pitcher_projection),
        )
        .route(
            "/api/pitcher-projections/:id",
            put(handlers::pitcher_projections::update_pitcher_projection),
        )
        .route(
            "/api/pitcher-projections/:id",
            delete(handlers::pitcher_projections::delete_pitcher_projection),
        )
        // Hitter Game routes
        .route(
            "/api/hitter-game",
            get(handlers::hitter_game::list_hitter_game),
        )
        .route(
            "/api/hitter-game",
            post(handlers::hitter_game::create_hitter_game),
        )
        .route(
            "/api/hitter-game/:id",
            get(handlers::hitter_game::get_hitter_game),
        )
        .route(
            "/api/hitter-game/:id",
            put(handlers::hitter_game::update_hitter_game),
        )
        .route(
            "/api/hitter-game/:id",
            delete(handlers::hitter_game::delete_hitter_game),
        )
        // Pitcher Game routes
        .route(
            "/api/pitcher-game",
            get(handlers::pitcher_game::list_pitcher_game),
        )
        .route(
            "/api/pitcher-game",
            post(handlers::pitcher_game::create_pitcher_game),
        )
        .route(
            "/api/pitcher-game/:id",
            get(handlers::pitcher_game::get_pitcher_game),
        )
        .route(
            "/api/pitcher-game/:id",
            put(handlers::pitcher_game::update_pitcher_game),
        )
        .route(
            "/api/pitcher-game/:id",
            delete(handlers::pitcher_game::delete_pitcher_game),
        )
        // Live Slates routes
        .route(
            "/api/live-slates",
            get(handlers::live_slates::list_live_slates),
        )
        .route(
            "/api/live-slates",
            post(handlers::live_slates::create_live_slate),
        )
        .route(
            "/api/live-slates/:id",
            get(handlers::live_slates::get_live_slate),
        )
        .route(
            "/api/live-slates/:id",
            put(handlers::live_slates::update_live_slate),
        )
        .route(
            "/api/live-slates/:id",
            delete(handlers::live_slates::delete_live_slate),
        )
        // DFN Live Hitters routes
        .route(
            "/api/dfn-live-hitters",
            get(handlers::dfn_live_hitters::list_dfn_live_hitters),
        )
        .route(
            "/api/dfn-live-hitters",
            post(handlers::dfn_live_hitters::create_dfn_live_hitter),
        )
        .route(
            "/api/dfn-live-hitters/:id",
            get(handlers::dfn_live_hitters::get_dfn_live_hitter),
        )
        .route(
            "/api/dfn-live-hitters/:id",
            put(handlers::dfn_live_hitters::update_dfn_live_hitter),
        )
        .route(
            "/api/dfn-live-hitters/:id",
            delete(handlers::dfn_live_hitters::delete_dfn_live_hitter),
        )
        // DFN Live Pitchers routes
        .route(
            "/api/dfn-live-pitchers",
            get(handlers::dfn_live_pitchers::list_dfn_live_pitchers),
        )
        .route(
            "/api/dfn-live-pitchers",
            post(handlers::dfn_live_pitchers::create_dfn_live_pitcher),
        )
        .route(
            "/api/dfn-live-pitchers/:id",
            get(handlers::dfn_live_pitchers::get_dfn_live_pitcher),
        )
        .route(
            "/api/dfn-live-pitchers/:id",
            put(handlers::dfn_live_pitchers::update_dfn_live_pitcher),
        )
        .route(
            "/api/dfn-live-pitchers/:id",
            delete(handlers::dfn_live_pitchers::delete_dfn_live_pitcher),
        )
        // MLB Live Lineups routes
        .route(
            "/api/mlb-live-lineups",
            get(handlers::mlb_live_lineups::list_mlb_live_lineups),
        )
        .route(
            "/api/mlb-live-lineups",
            post(handlers::mlb_live_lineups::create_mlb_live_lineup),
        )
        .route(
            "/api/mlb-live-lineups/:id",
            get(handlers::mlb_live_lineups::get_mlb_live_lineup),
        )
        .route(
            "/api/mlb-live-lineups/:id",
            put(handlers::mlb_live_lineups::update_mlb_live_lineup),
        )
        .route(
            "/api/mlb-live-lineups/:id",
            delete(handlers::mlb_live_lineups::delete_mlb_live_lineup),
        )
        // Contests Pull routes
        .route(
            "/api/contests-pull",
            get(handlers::contests_pull::list_contests_pull),
        )
        .route(
            "/api/contests-pull",
            post(handlers::contests_pull::create_contests_pull),
        )
        .route(
            "/api/contests-pull/:id",
            get(handlers::contests_pull::get_contests_pull),
        )
        .route(
            "/api/contests-pull/:id",
            put(handlers::contests_pull::update_contests_pull),
        )
        .route(
            "/api/contests-pull/:id",
            delete(handlers::contests_pull::delete_contests_pull),
        )
        // RPCs routes
        .route("/api/rpcs", get(handlers::rpcs::list_rpcs))
        .route("/api/rpcs", post(handlers::rpcs::create_rpc))
        .route("/api/rpcs/:id", get(handlers::rpcs::get_rpc))
        .route("/api/rpcs/:id", put(handlers::rpcs::update_rpc))
        .route("/api/rpcs/:id", delete(handlers::rpcs::delete_rpc))
        // Winning Charts routes
        .route(
            "/api/winning-charts",
            get(handlers::winning_charts::list_winning_charts),
        )
        .route(
            "/api/winning-charts",
            post(handlers::winning_charts::create_winning_chart),
        )
        .route(
            "/api/winning-charts/:id",
            get(handlers::winning_charts::get_winning_chart),
        )
        .route(
            "/api/winning-charts/:id",
            put(handlers::winning_charts::update_winning_chart),
        )
        .route(
            "/api/winning-charts/:id",
            delete(handlers::winning_charts::delete_winning_chart),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
