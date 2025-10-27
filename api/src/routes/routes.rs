// This file can be used to organize route definitions
// Currently routes are defined in main.rs but could be refactored here

use crate::AppState;
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    // Future: Move route definitions from main.rs here
    // This will make the codebase more maintainable
    Router::new()
}
