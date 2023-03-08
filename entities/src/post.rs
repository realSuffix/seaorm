//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

use crate::{author, post};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub text: String,
    pub author_id: Option<Uuid>,
    pub author2_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::author::Entity",
        from = "Column::Author2Id",
        to = "super::author::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Author2,
    #[sea_orm(
        belongs_to = "super::author::Entity",
        from = "Column::AuthorId",
        to = "super::author::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Author1,
}

impl ActiveModelBehavior for ActiveModel {}