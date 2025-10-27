use minoa::postgres::*;
use sea_orm::{ConnectionTrait, Database, DbBackend, Schema};

pub async fn create_schema(database_url: &str) -> Result<(), String> {
    println!("Connecting to database...");
    let db = Database::connect(database_url)
        .await
        .map_err(|e| format!("Failed to connect: {e}"))?;

    println!("Creating schema from minoa entities...\n");

    let schema = Schema::new(DbBackend::Postgres);

    // Create tables in dependency order
    let tables = vec![
        ("years", schema.create_table_from_entity(years::Entity)),
        ("dates", schema.create_table_from_entity(dates::Entity)),
        ("teams", schema.create_table_from_entity(teams::Entity)),
        ("venues", schema.create_table_from_entity(venues::Entity)),
        ("players", schema.create_table_from_entity(players::Entity)),
        ("games", schema.create_table_from_entity(games::Entity)),
        ("slates", schema.create_table_from_entity(slates::Entity)),
        (
            "dfs_players",
            schema.create_table_from_entity(dfs_players::Entity),
        ),
        ("weather", schema.create_table_from_entity(weather::Entity)),
        (
            "projections",
            schema.create_table_from_entity(projections::Entity),
        ),
        (
            "contest_list_parameters",
            schema.create_table_from_entity(contest_list_parameters::Entity),
        ),
        (
            "backtests",
            schema.create_table_from_entity(backtests::Entity),
        ),
        (
            "backtest_strategies",
            schema.create_table_from_entity(backtest_strategies::Entity),
        ),
        (
            "contest_data",
            schema.create_table_from_entity(contest_data::Entity),
        ),
        (
            "backtest_files",
            schema.create_table_from_entity(backtest_files::Entity),
        ),
        (
            "backtest_lineups",
            schema.create_table_from_entity(backtest_lineups::Entity),
        ),
        (
            "backtest_logs",
            schema.create_table_from_entity(backtest_logs::Entity),
        ),
        (
            "backtest_summaries",
            schema.create_table_from_entity(backtest_summaries::Entity),
        ),
        (
            "backtest_results",
            schema.create_table_from_entity(backtest_results::Entity),
        ),
        (
            "backtest_contest_list",
            schema.create_table_from_entity(backtest_contest_list::Entity),
        ),
        (
            "backtest_contest_summaries",
            schema.create_table_from_entity(backtest_contest_summaries::Entity),
        ),
        (
            "backtests_complete",
            schema.create_table_from_entity(backtests_complete::Entity),
        ),
        (
            "hitter_projections",
            schema.create_table_from_entity(hitter_projections::Entity),
        ),
        (
            "pitcher_projections",
            schema.create_table_from_entity(pitcher_projections::Entity),
        ),
        (
            "hitter_game",
            schema.create_table_from_entity(hitter_game::Entity),
        ),
        (
            "pitcher_game",
            schema.create_table_from_entity(pitcher_game::Entity),
        ),
        (
            "live_slates",
            schema.create_table_from_entity(live_slates::Entity),
        ),
        (
            "dfn_live_hitters",
            schema.create_table_from_entity(dfn_live_hitters::Entity),
        ),
        (
            "dfn_live_pitchers",
            schema.create_table_from_entity(dfn_live_pitchers::Entity),
        ),
        (
            "mlb_live_lineups",
            schema.create_table_from_entity(mlb_live_lineups::Entity),
        ),
        (
            "contests_pull",
            schema.create_table_from_entity(contests_pull::Entity),
        ),
        ("rpcs", schema.create_table_from_entity(rpcs::Entity)),
        (
            "winning_charts",
            schema.create_table_from_entity(winning_charts::Entity),
        ),
        (
            "goose_db_version",
            schema.create_table_from_entity(goose_db_version::Entity),
        ),
    ];

    let mut created = 0;
    let mut skipped = 0;

    for (table_name, table) in tables {
        let stmt = db.get_database_backend().build(&table);

        match db.execute(stmt).await {
            Ok(_) => {
                println!("✓ Created table: {table_name}");
                created += 1;
            }
            Err(e) if e.to_string().contains("already exists") => {
                println!("⚠ Table already exists: {table_name}");
                skipped += 1;
            }
            Err(e) => {
                return Err(format!("Failed to create {table_name}: {e}"));
            }
        }
    }

    println!("\n✅ Schema creation complete! (Created: {created}, Skipped: {skipped})",);
    Ok(())
}

pub async fn check_schema(database_url: &str) -> Result<(), String> {
    let db = Database::connect(database_url)
        .await
        .map_err(|e| format!("Failed to connect: {e}"))?;

    let sql = r#"
        SELECT table_name 
        FROM information_schema.tables 
        WHERE table_schema = 'public' 
        AND table_type = 'BASE TABLE'
        ORDER BY table_name
    "#;

    let result = db
        .query_all(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            sql,
        ))
        .await
        .map_err(|e| format!("Failed to query tables: {e}"))?;

    if result.is_empty() {
        println!("❌ No tables found. Run: cli db create");
    } else {
        println!("Found {} tables:\n", result.len());
        for row in result {
            let table_name: String = row
                .try_get("", "table_name")
                .unwrap_or_else(|_| "unknown".to_string());
            println!("  • {table_name}");
        }
    }

    Ok(())
}

pub async fn drop_schema(database_url: &str, confirm: bool) -> Result<(), String> {
    if !confirm {
        return Err("Schema drop requires --confirm flag".to_string());
    }

    println!("⚠️  WARNING: Dropping all tables!");
    let db = Database::connect(database_url)
        .await
        .map_err(|e| format!("Failed to connect: {e}"))?;

    // Drop in reverse order
    let tables = vec![
        "goose_db_version",
        "winning_charts",
        "rpcs",
        "contests_pull",
        "mlb_live_lineups",
        "dfn_live_pitchers",
        "dfn_live_hitters",
        "live_slates",
        "pitcher_game",
        "hitter_game",
        "pitcher_projections",
        "hitter_projections",
        "backtests_complete",
        "backtest_contest_summaries",
        "backtest_contest_list",
        "backtest_results",
        "backtest_summaries",
        "backtest_logs",
        "backtest_lineups",
        "backtest_files",
        "contest_data",
        "backtest_strategies",
        "backtests",
        "contest_list_parameters",
        "projections",
        "weather",
        "dfs_players",
        "slates",
        "games",
        "players",
        "venues",
        "teams",
        "dates",
        "years",
        "seaql_migrations",
    ];

    for table in tables {
        let sql = format!("DROP TABLE IF EXISTS {table} CASCADE");
        let _ = db.execute_unprepared(&sql).await;
        println!("✓ Dropped: {table}");
    }

    println!("\n✅ All tables dropped");
    Ok(())
}
