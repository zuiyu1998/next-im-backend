use entity::msg::{MsgColumn, MsgEntity};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MsgEntity)
                    .if_not_exists()
                    .col(pk_auto(MsgColumn::Id))
                    .col(string(MsgColumn::LocalId))
                    .col(big_integer(MsgColumn::ServerAt))
                    .col(string(MsgColumn::ServerId))
                    .col(big_integer(MsgColumn::ServerAt))
                    .col(big_integer(MsgColumn::SeqId))
                    .col(big_integer(MsgColumn::SenderId))
                    .col(big_integer(MsgColumn::ReceiverId))
                    .col(string(MsgColumn::MsgType))
                    .col(binary(MsgColumn::Content))
                    .col(string(MsgColumn::ChatType))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MsgEntity).to_owned())
            .await
    }
}
