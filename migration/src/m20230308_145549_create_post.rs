use sea_orm_migration::prelude::*;

use crate::m20230308_145516_create_author::Author;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::AuthorId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("author_id_fk")
                            .from(Post::Table, Post::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .col(ColumnDef::new(Post::Author2Id).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("author2_id_fk")
                            .from(Post::Table, Post::Author2Id)
                            .to(Author::Table, Author::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Post {
    Table,
    Id,
    Title,
    Text,
    AuthorId,
    Author2Id,
}
