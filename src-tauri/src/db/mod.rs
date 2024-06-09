use crate::CONFIG_DIR;
use sqlx::migrate::MigrateDatabase;
use tauri_plugin_sql::{Migration, MigrationKind};

pub mod greetings;

pub type Pool = sqlx::Pool<sqlx::Sqlite>;

pub fn db_url() -> String {
    #[cfg(not(debug_assertions))] // release
    {
        env!("DATABASE_URL").to_string()
    }

    #[cfg(debug_assertions)] // non-release
    {
        format!("{}/db.sqlite", CONFIG_DIR.to_string_lossy())
    }
}

pub fn new_pool() -> Pool {
    let db_url = db_url();
    sqlx::SqlitePool::connect_lazy(&db_url).unwrap()
}

pub fn setup_builder() -> tauri_plugin_sql::Builder {
    let db_url = db_url();
    let migrations = migrations();

    tauri_plugin_sql::Builder::new().add_migrations(&db_url, migrations)
}

fn migrations() -> Vec<Migration> {
    use include_dir::{include_dir, Dir};
    static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

    PROJECT_DIR
        .files()
        .filter(|v| v.path().is_file())
        .filter_map(create_migration_from_file)
        .collect()
}

fn create_migration_from_file(file: &'static include_dir::File) -> Option<Migration> {
    let filename = file.path().file_name()?.to_str()?;
    let file_parts: Vec<&str> = filename.split('.').collect();

    // Extract <version>_<description>.<up|down>.sql
    let (mut ver_desc, kind) = match file_parts[..] {
        [ver_desc, kind, _ext] => (ver_desc.split('_'), kind),
        [ver_desc, _ext] => (ver_desc.split('_'), "up"),
        _ => return None,
    };

    let migration = Migration {
        version: ver_desc.next()?.parse().ok()?,
        description: ver_desc.next()?,
        kind: match kind {
            "down" => MigrationKind::Down,
            _ => MigrationKind::Up,
        },
        sql: file.contents_utf8()?,
    };

    Some(migration)
}

pub async fn create_if_none() -> Result<(), sqlx::Error> {
    let db_url = db_url();

    if !sqlx::Sqlite::database_exists(&db_url).await? {
        sqlx::Sqlite::create_database(&db_url).await?;
    }

    Ok(())
}
