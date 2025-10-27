use prettytable::{format, Cell, Row, Table};
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, Statement};

pub async fn show_table_counts(db: &DatabaseConnection) -> Result<(), String> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("Table Name"),
        Cell::new("Row Count"),
    ]));

    let table_names = vec![
        "players",
        "games",
        "teams",
        "venues",
        "backtests",
        "backtest_files",
        "backtest_lineups",
        "backtest_summaries",
        "backtest_strategies",
        "contest_data",
        "dfs_players",
        "hitter_game",
        "pitcher_game",
        "slates",
        "weather",
    ];

    for table_name in table_names {
        let sql = format!("SELECT COUNT(*) FROM {table_name}");
        match db
            .query_one(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                sql,
            ))
            .await
        {
            Ok(Some(result)) => {
                if let Ok(count) = result.try_get_by_index::<i64>(0) {
                    table.add_row(Row::new(vec![
                        Cell::new(table_name),
                        Cell::new(&format!("{count:>12}")),
                    ]));
                }
            }
            Ok(None) => {
                eprintln!("Warning: No result for {table_name}");
            }
            Err(e) => {
                eprintln!("Warning: Failed to count {table_name}: {e}");
            }
        }
    }

    table.printstd();
    Ok(())
}

pub async fn show_backtest_stats(
    db: &DatabaseConnection,
    backtest_id: Option<i32>,
) -> Result<(), String> {
    use minoa::postgres::backtest_summaries::{Column, Entity as BacktestSummary};
    use sea_orm::{ColumnTrait, QueryFilter};

    let mut query = BacktestSummary::find();

    if let Some(id) = backtest_id {
        query = query.filter(Column::BacktestId.eq(id));
    }

    let summaries = query
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch backtest stats: {e}"))?;

    if summaries.is_empty() {
        println!("No backtest summaries found.");
        return Ok(());
    }

    // Calculate aggregate statistics
    let total_contests: i32 = summaries.iter().map(|s| s.contests).sum();
    let total_profit: i32 = summaries.iter().map(|s| s.profit).sum();
    let total_cost: i32 = summaries.iter().map(|s| s.total_cost).sum();
    let total_prizes: i32 = summaries.iter().map(|s| s.total_prizes).sum();
    let avg_roi: f64 =
        summaries.iter().map(|s| s.roi_percent).sum::<f64>() / summaries.len() as f64;
    let total_first_places: i32 = summaries.iter().map(|s| s.total_first_places).sum();
    let total_top_5: i32 = summaries.iter().map(|s| s.total_top_5).sum();
    let total_top_10: i32 = summaries.iter().map(|s| s.total_top_10).sum();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![Cell::new("Metric"), Cell::new("Value")]));

    table.add_row(Row::new(vec![
        Cell::new("Total Summaries"),
        Cell::new(&summaries.len().to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total Contests"),
        Cell::new(&total_contests.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total Profit"),
        Cell::new(&format!("${total_profit}")),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total Cost"),
        Cell::new(&format!("${total_cost}")),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total Prizes"),
        Cell::new(&format!("${total_prizes}")),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Average ROI"),
        Cell::new(&format!("{avg_roi:.2}%")),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total 1st Places"),
        Cell::new(&total_first_places.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total Top 5"),
        Cell::new(&total_top_5.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total Top 10"),
        Cell::new(&total_top_10.to_string()),
    ]));

    table.printstd();
    Ok(())
}

pub async fn show_player_stats(db: &DatabaseConnection, min_games: i32) -> Result<(), String> {
    use sea_orm::DbBackend;

    let sql = format!(
        r#"
        SELECT 
            p.player_id,
            p.player_name,
            COUNT(DISTINCT hg.game_id) as hitter_games,
            COUNT(DISTINCT pg.game_id) as pitcher_games
        FROM players p
        LEFT JOIN hitter_game hg ON p.player_id = hg.player_id
        LEFT JOIN pitcher_game pg ON p.player_id = pg.player_id
        GROUP BY p.player_id, p.player_name
        HAVING COUNT(DISTINCT hg.game_id) >= {min_games} OR COUNT(DISTINCT pg.game_id) >= {min_games}
        ORDER BY (COUNT(DISTINCT hg.game_id) + COUNT(DISTINCT pg.game_id)) DESC
        LIMIT 50
        "#
    );

    let results = db
        .query_all(Statement::from_string(DbBackend::Postgres, sql))
        .await
        .map_err(|e| format!("Failed to fetch player stats: {e}"))?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("Player ID"),
        Cell::new("Player Name"),
        Cell::new("Hitter Games"),
        Cell::new("Pitcher Games"),
        Cell::new("Total Games"),
    ]));

    for row in results {
        let player_id: i32 = row.try_get("", "player_id").unwrap_or(0);
        let player_name: String = row
            .try_get("", "player_name")
            .unwrap_or_else(|_| "N/A".to_string());
        let hitter_games: i64 = row.try_get("", "hitter_games").unwrap_or(0);
        let pitcher_games: i64 = row.try_get("", "pitcher_games").unwrap_or(0);
        let total = hitter_games + pitcher_games;

        table.add_row(Row::new(vec![
            Cell::new(&player_id.to_string()),
            Cell::new(&player_name),
            Cell::new(&hitter_games.to_string()),
            Cell::new(&pitcher_games.to_string()),
            Cell::new(&total.to_string()),
        ]));
    }

    table.printstd();
    Ok(())
}

pub async fn show_database_size(db: &DatabaseConnection) -> Result<(), String> {
    use sea_orm::DbBackend;

    let sql = r#"
        SELECT 
            pg_size_pretty(pg_database_size(current_database())) as db_size,
            pg_database_size(current_database()) as db_size_bytes
    "#;

    let result = db
        .query_one(Statement::from_string(DbBackend::Postgres, sql.to_string()))
        .await
        .map_err(|e| format!("Failed to fetch database size: {e}"))?;

    if let Some(row) = result {
        let db_size: String = row
            .try_get("", "db_size")
            .unwrap_or_else(|_| "N/A".to_string());
        let db_size_bytes: i64 = row.try_get("", "db_size_bytes").unwrap_or(0);

        println!("┌─────────────────────────────────────┐");
        println!("│  Database Size Information          │");
        println!("├─────────────────────────────────────┤");
        println!("│  Total Size: {db_size:<22} │");
        println!("│  Size (bytes): {db_size_bytes:<20} │");
        println!("└─────────────────────────────────────┘");
        println!();
    }

    // Get table sizes
    let table_sql = r#"
        SELECT 
            tablename,
            pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) AS size,
            pg_total_relation_size(schemaname||'.'||tablename) AS size_bytes
        FROM pg_tables
        WHERE schemaname = 'public'
        ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC
        LIMIT 15
    "#;

    let table_results = db
        .query_all(Statement::from_string(
            DbBackend::Postgres,
            table_sql.to_string(),
        ))
        .await
        .map_err(|e| format!("Failed to fetch table sizes: {e}"))?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![Cell::new("Table Name"), Cell::new("Size")]));

    for row in table_results {
        let tablename: String = row
            .try_get("", "tablename")
            .unwrap_or_else(|_| "N/A".to_string());
        let size: String = row
            .try_get("", "size")
            .unwrap_or_else(|_| "N/A".to_string());

        table.add_row(Row::new(vec![Cell::new(&tablename), Cell::new(&size)]));
    }

    println!("Top 15 Tables by Size:");
    table.printstd();

    Ok(())
}
