pub use sea_orm_migration::prelude::*;

mod m20250221_075209_create_table_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250221_075209_create_table_user::Migration)]
    }
}
