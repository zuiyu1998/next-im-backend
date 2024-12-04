use entity::seq::{SeqColumn, SeqEntity};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SeqEntity)
                    .if_not_exists()
                    .col(big_integer(SeqColumn::SeqId))
                    .col(integer(SeqColumn::ChartType))
                    .col(big_integer(SeqColumn::SenderId))
                    .col(big_integer(SeqColumn::ReceiverId))
                    .primary_key(
                        Index::create()
                            .col(SeqColumn::ChartType)
                            .col(SeqColumn::SenderId)
                            .col(SeqColumn::ReceiverId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SeqEntity).to_owned())
            .await
    }
}
