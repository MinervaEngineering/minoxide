mod commands;

use clap::{Parser, Subcommand};
use sea_orm::{Database, DatabaseConnection};
use std::process;

#[derive(Parser)]
#[command(name = "minoa-cli")]
#[command(about = "A CLI tool for managing the Minoa PostgreSQL database", long_about = None)]
struct Cli {
    /// Database connection string
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init database schema
    Db {
        #[command(subcommand)]
        action: DbCommands,
    },
    /// Query database tables
    Query {
        #[command(subcommand)]
        query_type: QueryCommands,
    },
    /// Display database statistics
    Stats {
        #[command(subcommand)]
        stats_type: StatsCommands,
    },
    /// Run database migrations
    Migrate {
        /// Migration direction (up/down)
        #[arg(value_enum)]
        direction: MigrateDirection,
        /// Number of migrations to run
        #[arg(short, long, default_value = "1")]
        steps: u32,
    },
    /// Backup database tables
    Backup {
        /// Tables to backup (comma-separated, or 'all')
        #[arg(short, long)]
        tables: String,
        /// Output directory for backups
        #[arg(short, long, default_value = "./backups")]
        output: String,
    },
    /// Validate data integrity
    Validate {
        /// Run full validation including foreign keys
        #[arg(short, long)]
        full: bool,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Create all tables from minoa entities
    Create,

    /// Check existing database schema
    Check,

    /// Drop all tables in the database
    Drop {
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
enum QueryCommands {
    /// List all players
    Players {
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u64>,
    },
    /// List all games
    Games {
        /// Filter by date (YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u64>,
    },
    /// List all backtests
    Backtests {
        /// Show only completed backtests
        #[arg(short, long)]
        completed: bool,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u64>,
    },
    /// Query backtest summaries
    BacktestSummaries {
        /// Filter by backtest ID
        #[arg(short, long)]
        backtest_id: Option<i32>,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u64>,
    },
    /// Query contests
    Contests {
        /// Filter by slate ID
        #[arg(short, long)]
        slate_id: Option<i32>,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u64>,
    },
}

#[derive(Subcommand)]
enum StatsCommands {
    /// Show table row counts
    Tables,
    /// Show backtest performance stats
    Backtests {
        /// Specific backtest ID to analyze
        #[arg(short, long)]
        id: Option<i32>,
    },
    /// Show player statistics
    Players {
        /// Minimum number of games played
        #[arg(short, long, default_value = "10")]
        min_games: i32,
    },
    /// Show database size information
    Database,
}

#[derive(clap::ValueEnum, Clone)]
enum MigrateDirection {
    Up,
    Down,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Connect to database
    let db = match Database::connect(&cli.database_url).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error connecting to database: {}", err);
            process::exit(1);
        }
    };

    let result = match cli.command {
        Commands::Db { action } => match action {
            DbCommands::Create => commands::db::create_schema(&cli.database_url).await,
            DbCommands::Check => commands::db::check_schema(&cli.database_url).await,
            DbCommands::Drop { confirm } => {
                commands::db::drop_schema(&cli.database_url, confirm).await
            }
        },
        Commands::Query { query_type } => handle_query(&db, query_type).await,
        Commands::Stats { stats_type } => handle_stats(&db, stats_type).await,
        Commands::Migrate { direction, steps } => handle_migrate(&db, direction, steps).await,
        Commands::Backup { tables, output } => handle_backup(&db, tables, output).await,
        Commands::Validate { full } => handle_validate(&db, full).await,
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

async fn handle_query(db: &DatabaseConnection, query_type: QueryCommands) -> Result<(), String> {
    match query_type {
        QueryCommands::Players { limit } => commands::query::list_players(db, limit).await,
        QueryCommands::Games { date, limit } => commands::query::list_games(db, date, limit).await,
        QueryCommands::Backtests { completed, limit } => {
            commands::query::list_backtests(db, completed, limit).await
        }
        QueryCommands::BacktestSummaries { backtest_id, limit } => {
            commands::query::list_backtest_summaries(db, backtest_id, limit).await
        }
        QueryCommands::Contests { slate_id, limit } => {
            commands::query::list_contests(db, slate_id, limit).await
        }
    }
}

async fn handle_stats(db: &DatabaseConnection, stats_type: StatsCommands) -> Result<(), String> {
    match stats_type {
        StatsCommands::Tables => commands::stats::show_table_counts(db).await,
        StatsCommands::Backtests { id } => commands::stats::show_backtest_stats(db, id).await,
        StatsCommands::Players { min_games } => {
            commands::stats::show_player_stats(db, min_games).await
        }
        StatsCommands::Database => commands::stats::show_database_size(db).await,
    }
}

async fn handle_migrate(
    db: &DatabaseConnection,
    direction: MigrateDirection,
    steps: u32,
) -> Result<(), String> {
    commands::migrate::run_migration(db, direction, steps).await
}

async fn handle_backup(
    db: &DatabaseConnection,
    tables: String,
    output: String,
) -> Result<(), String> {
    commands::backup::create_backup(db, tables, output).await
}

async fn handle_validate(db: &DatabaseConnection, full: bool) -> Result<(), String> {
    commands::validate::validate_database(db, full).await
}
