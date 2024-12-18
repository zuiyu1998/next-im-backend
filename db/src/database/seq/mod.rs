use crate::seq::SeqRepo;
use abi::{
    async_trait::async_trait,
    error::ErrorKind,
    pb::message::Sequence,
    sea_orm::{EntityTrait, QueryFilter},
    Result,
};
use entity::{
    sea_orm::{entity::Set, ActiveModelTrait, ColumnTrait, DatabaseConnection},
    seq::{SeqActiveModel, SeqColumn, SeqEntity},
};

#[derive(Debug)]
pub struct SeqDb {
   pub(crate) conn: DatabaseConnection,
}

#[async_trait]
impl SeqRepo for SeqDb {
    //读取序列号
    async fn read_sequence_id(&self, sequence: &Sequence) -> Result<i64> {
        let sql = SeqEntity::find()
            .filter(SeqColumn::ChartType.eq(sequence.chat_type))
            .filter(SeqColumn::SenderId.eq(sequence.sender_id))
            .filter(SeqColumn::ReceiverId.eq(sequence.receiver_id));

        let model = sql.one(&self.conn).await?.ok_or(ErrorKind::SeqNotFound)?;

        Ok(model.seq_id)
    }
    //存储序列号
    async fn update_sequence_id(&self, sequence: &Sequence, id: i64) -> Result<i64> {
        let mut active = <SeqActiveModel as ActiveModelTrait>::default();
        active.chart_type = Set(sequence.chat_type);
        active.sender_id = Set(sequence.sender_id);
        active.receiver_id = Set(sequence.receiver_id);
        active.seq_id = Set(id);

        let model = active.update(&self.conn).await?;

        Ok(model.seq_id)
    }

    //创建序列号
    async fn create_sequence_id(&self, sequence: &Sequence) -> Result<i64> {
        let mut active = <SeqActiveModel as ActiveModelTrait>::default();
        active.chart_type = Set(sequence.chat_type);
        active.sender_id = Set(sequence.sender_id);
        active.receiver_id = Set(sequence.receiver_id);
        active.seq_id = Set(1);

        let model = active.insert(&self.conn).await?;

        Ok(model.seq_id)
    }
}
