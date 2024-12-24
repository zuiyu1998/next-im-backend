use abi::sea_orm::{entity::prelude::*, self};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "next_msg")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub local_id: String,
    pub local_at: i64,
    pub server_id: String,
    pub server_at: i64,
    pub seq_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub msg_type: String,
    pub content: Vec<u8>,
    pub chat_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}