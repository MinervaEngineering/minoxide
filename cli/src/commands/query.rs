use minoa::postgres::prelude::*;
use prettytable::{format, Cell, Row, Table};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

pub async fn list_players(db: &DatabaseConnection, limit: Option<u64>) -> Result<(), String> {
    use minoa::postgres::players::{Column, Entity as Player};

    let mut query = Player::find();

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let players = query
        .order_by_asc(Column::PlayerId)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch players: {}", e))?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Player Name"),
        Cell::new("First Name"),
        Cell::new("Last Name"),
    ]));

    for player in players {
        table.add_row(Row::new(vec![
            Cell::new(&player.player_id.to_string()),
            Cell::new(&player.player_name.unwrap_or_else(|| "N/A".to_string())),
            Cell::new(&player.first_name.unwrap_or_else(|| "N/A".to_string())),
            Cell::new(&player.last_name.unwrap_or_else(|| "N/A".to_string())),
        ]));
    }

    table.printstd();
    println!("\nTotal: {} players", table.len());
    Ok(())
}

pub async fn list_games(
    db: &DatabaseConnection,
    date_filter: Option<String>,
    limit: Option<u64>,
) -> Result<(), String> {
    use minoa::postgres::games::{Column, Entity as Game};

    let mut query = Game::find();

    if let Some(_date_str) = date_filter {
        println!("Note: Date filtering requires joining with dates table");
        // TODO: Implement date filtering with joins
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let games = query
        .order_by_desc(Column::GameId)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch games: {}", e))?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("Game ID"),
        Cell::new("Date ID"),
        Cell::new("Status"),
        Cell::new("Away"),
        Cell::new("Home"),
        Cell::new("Score"),
    ]));

    for game in games {
        let score = format!(
            "{}-{}",
            game.away_runs.map_or("?".to_string(), |r| r.to_string()),
            game.home_runs.map_or("?".to_string(), |r| r.to_string())
        );

        table.add_row(Row::new(vec![
            Cell::new(&game.game_id.to_string()),
            Cell::new(&game.date_id.map_or("N/A".to_string(), |d| d.to_string())),
            Cell::new(&game.status.unwrap_or_else(|| "N/A".to_string())),
            Cell::new(&game.away_id.map_or("N/A".to_string(), |d| d.to_string())),
            Cell::new(&game.home_id.map_or("N/A".to_string(), |d| d.to_string())),
            Cell::new(&score),
        ]));
    }

    table.printstd();
    println!("\nTotal: {} games", table.len());
    Ok(())
}

pub async fn list_backtests(
    db: &DatabaseConnection,
    completed_only: bool,
    limit: Option<u64>,
) -> Result<(), String> {
    use minoa::postgres::backtests::{Column, Entity as Backtest};

    let mut query = Backtest::find();

    if completed_only {
        query = query.filter(Column::HasLineups.eq(true));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let backtests = query
        .order_by_desc(Column::Id)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch backtests: {}", e))?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Contest Serial"),
        Cell::new("Season"),
        Cell::new("Stacking Type"),
        Cell::new("Has Lineups"),
    ]));

    for bt in &backtests {
        table.add_row(Row::new(vec![
            Cell::new(&bt.id.to_string()),
            Cell::new(&bt.contest_serial.to_string()),
            Cell::new(&bt.season_year),
            Cell::new(&bt.stacking_type),
            Cell::new(if bt.has_lineups { "✓" } else { "✗" }),
        ]));
    }

    table.printstd();
    println!("\nTotal: {} backtests", backtests.len());
    Ok(())
}

pub async fn list_backtest_summaries(
    db: &DatabaseConnection,
    backtest_id: Option<i32>,
    limit: Option<u64>,
) -> Result<(), String> {
    use minoa::postgres::backtest_summaries::{Column, Entity as BacktestSummary};

    let mut query = BacktestSummary::find();

    if let Some(id) = backtest_id {
        query = query.filter(Column::BacktestId.eq(id));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let summaries = query
        .order_by_desc(Column::Id)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch backtest summaries: {}", e))?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Backtest"),
        Cell::new("Contests"),
        Cell::new("Profit"),
        Cell::new("ROI %"),
        Cell::new("1st"),
        Cell::new("Top 5"),
    ]));

    for summary in &summaries {
        table.add_row(Row::new(vec![
            Cell::new(&summary.id.to_string()),
            Cell::new(&summary.backtest_id.to_string()),
            Cell::new(&summary.contests.to_string()),
            Cell::new(&format!("${}", summary.profit)),
            Cell::new(&format!("{:.2}", summary.roi_percent)),
            Cell::new(&summary.total_first_places.to_string()),
            Cell::new(&summary.total_top_5.to_string()),
        ]));
    }

    table.printstd();
    println!("\nTotal: {} summaries", summaries.len());
    Ok(())
}

pub async fn list_contests(
    db: &DatabaseConnection,
    slate_id: Option<i32>,
    limit: Option<u64>,
) -> Result<(), String> {
    use minoa::postgres::contest_data::{Column, Entity as Contest};

    let mut query = Contest::find();

    if let Some(sid) = slate_id {
        query = query.filter(Column::SlateId.eq(sid));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let contests = query
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch contests: {}", e))?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("Contest ID"),
        Cell::new("Name"),
        Cell::new("Entry Fee"),
        Cell::new("Entries"),
        Cell::new("Prize Pool"),
    ]));

    for contest in &contests {
        table.add_row(Row::new(vec![
            Cell::new(&contest.contest_id),
            Cell::new(&contest.contest_name.as_ref().unwrap_or(&"N/A".to_string())),
            Cell::new(
                &contest
                    .entry_fee
                    .map_or("N/A".to_string(), |f| format!("${:.2}", f)),
            ),
            Cell::new(
                &contest
                    .total_entries
                    .map_or("N/A".to_string(), |e| e.to_string()),
            ),
            Cell::new(
                &contest
                    .prize_pool
                    .map_or("N/A".to_string(), |p| format!("${}", p)),
            ),
        ]));
    }

    table.printstd();
    println!("\nTotal: {} contests", contests.len());
    Ok(())
}
