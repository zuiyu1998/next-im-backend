use abi::sea_orm::{entity::prelude::*, self};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "next_sequence")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub chart_type: i32,
    #[sea_orm(primary_key)]
    pub sender_id: i64,
    #[sea_orm(primary_key)]
    pub receiver_id: i64,
    pub seq_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}