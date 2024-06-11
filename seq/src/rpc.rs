use abi::{
    pb::message::{sequence_service_server::SequenceService, Sequence, SequenceResponse},
    tonic::{async_trait, Request, Response, Status},
};

use crate::SeqRpcService;

#[async_trait]
impl SequenceService for SeqRpcService {
    async fn get_sequence_id(
        &self,
        request: Request<Sequence>,
    ) -> std::result::Result<Response<SequenceResponse>, Status> {
        let req = request.into_inner();

        let id = self.get_id(req).await?;

        Ok(Response::new(SequenceResponse { id }))
    }
}
