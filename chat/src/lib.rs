use abi::{
    nanoid::{self, nanoid},
    pb::message::{chat_service_server::ChatService, ChatMsg, MsgResponse},
    tonic::{async_trait, Request, Response, Status},
    tracing,
};
use rdkafka::producer::{FutureProducer, FutureRecord};

pub struct ChatRpcService {
    kafka: FutureProducer,
    topic: String,
}

impl ChatService for ChatRpcService {
    async fn send_message(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<MsgResponse>, Status> {
        let mut msg = request.into_inner();

        msg.server_id = nanoid!();

        msg.server_at = chrono::Local::now()
            .naive_local()
            .and_utc()
            .timestamp_millis();

        // send msg to kafka
        let payload = serde_json::to_string(&msg).unwrap();
        // let kafka generate key, then we need set FutureRecord<String, type>
        let record: FutureRecord<String, String> = FutureRecord::to(&self.topic).payload(&payload);

        tracing::info!("send msg to kafka: {:?}", record);

        let err = match self.kafka.send(record, Duration::from_secs(0)).await {
            Ok(_) => String::new(),
            Err((err, msg)) => {
                error!(
                    "send msg to kafka error: {:?}; owned message: {:?}",
                    err, msg
                );
                err.to_string()
            }
        };

        return Ok(tonic::Response::new(MsgResponse {
            local_id: msg.local_id,
            server_id: msg.server_id,
            server_at: msg.server_at,
            err,
        }));
    }
}
