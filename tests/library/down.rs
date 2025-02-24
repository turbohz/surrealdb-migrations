use anyhow::{ensure, Result};
use assert_fs::TempDir;
use surrealdb_migrations::MigrationRunner;

use crate::helpers::*;

#[tokio::test]
async fn apply_revert_all_migrations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_name = generate_random_db_name()?;

    add_migration_config_file_with_db_name_in_dir(&temp_dir, DbInstance::Root, &db_name)?;
    scaffold_blog_template(&temp_dir)?;

    let config_file_path = temp_dir.join(".surrealdb");

    let configuration = SurrealdbConfiguration {
        db: Some(db_name.to_string()),
        ..Default::default()
    };

    let db = create_surrealdb_client(&configuration).await?;

    let runner =
        MigrationRunner::new(&db).use_config_file(config_file_path.to_str().unwrap_or_default());

    runner.up().await?;

    runner.down("0").await?;

    let migrations_applied = runner.list().await?;
    ensure!(migrations_applied.len() == 0);

    Ok(())
}

#[tokio::test]
async fn apply_revert_to_first_migration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_name = generate_random_db_name()?;

    add_migration_config_file_with_db_name_in_dir(&temp_dir, DbInstance::Root, &db_name)?;
    scaffold_blog_template(&temp_dir)?;

    let first_migration_name = get_first_migration_name(&temp_dir)?;

    let config_file_path = temp_dir.join(".surrealdb");

    let configuration = SurrealdbConfiguration {
        db: Some(db_name.to_string()),
        ..Default::default()
    };
    let db = create_surrealdb_client(&configuration).await?;

    let runner =
        MigrationRunner::new(&db).use_config_file(config_file_path.to_str().unwrap_or_default());

    runner.up().await?;

    runner.down(&first_migration_name).await?;

    let migrations_applied = runner.list().await?;
    ensure!(migrations_applied.len() == 1);

    Ok(())
}
