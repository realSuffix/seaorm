pub use sea_orm_migration::prelude::*;

mod m20230308_145516_create_author;
mod m20230308_145549_create_post;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230308_145516_create_author::Migration),
            Box::new(m20230308_145549_create_post::Migration),
        ]
    }
}
