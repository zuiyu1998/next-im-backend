pub use sea_orm_migration::prelude::*;

mod m20241224_013252_create_msg;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241224_013252_create_msg::Migration),
        ]
    }
}
