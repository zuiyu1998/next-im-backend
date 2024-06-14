use abi::{
    pb::message::{db_service_server::DbService, Sequence, SequenceResponse},
    tonic::{async_trait, Request, Response, Status},
};

use super::DbRpcService;

#[async_trait]
impl DbService for DbRpcService {
    async fn read_sequence_id(
        &self,
        request: Request<Sequence>,
    ) -> Result<Response<SequenceResponse>, Status> {
        todo!()
    }

    async fn store_sequence_id(
        &self,
        request: Request<Sequence>,
    ) -> Result<Response<SequenceResponse>, Status> {
        todo!()
    }
}
