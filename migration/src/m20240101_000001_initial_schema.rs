use sea_orm_migration::prelude::*;

/// Baseline migration - assumes existing schema from minoa entities.
/// This migration does nothing but establishes the migration tracking baseline.
/// Your existing database schema (generated from minoa/src/postgres entities) is already in place.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No-op: Schema already exists from minoa entities
        // This migration just initializes the migration tracking system
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No-op: We don't want to drop existing tables
        Ok(())
    }
}
