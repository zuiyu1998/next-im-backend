use abi::{
    pb::message::{db_service_server::DbService, Sequence, SequenceResponse, SequenceUpdate},
    tonic::{async_trait, Request, Response, Status},
};

use super::DbRpcService;

#[async_trait]
impl DbService for DbRpcService {
    async fn read_sequence_id(
        &self,
        request: Request<Sequence>,
    ) -> Result<Response<SequenceResponse>, Status> {
        let sequence = request.into_inner();

        let id = self.seq.read_sequence_id(&sequence).await?;

        Ok(Response::new(SequenceResponse { id }))
    }

    async fn update_sequence_id(
        &self,
        request: Request<SequenceUpdate>,
    ) -> Result<Response<SequenceResponse>, Status> {
        let update = request.into_inner();

        let sequence = update.sequence.unwrap();
        let id = self.seq.update_sequence_id(&sequence, update.id).await?;

        Ok(Response::new(SequenceResponse { id }))
    }

    async fn create_sequence_id(
        &self,
        request: Request<Sequence>,
    ) -> Result<Response<SequenceResponse>, Status> {
        let sequence = request.into_inner();

        let id = self.seq.create_sequence_id(&sequence).await?;

        Ok(Response::new(SequenceResponse { id }))
    }
}
