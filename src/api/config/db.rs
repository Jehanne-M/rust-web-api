use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::{env, time::Duration};

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db_pass found failed");

    let mut opt = ConnectOptions::new(db_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let db_conn = Database::connect(opt).await?;

    Migrator::up(&db_conn, None).await?;

    Ok(db_conn)
}
