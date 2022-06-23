use tonic::{Request, Response, Status};
use rpc::account_server::{Account};
use rpc::{Reply, RegisterRequest, RegisterReply};

pub mod rpc {
    tonic::include_proto!("top.l1men9yu.yura");
}

#[derive(Debug, Default)]
pub struct RPCCenter {}

#[tonic::async_trait]
impl Account for RPCCenter {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterReply>, Status> {
        println!("Got a request: {:?}", request);

        let register_reply = rpc::RegisterReply {
            reply: Some(
                Reply {
                    code: 0,
                    message: Some("success".to_string()),
                }
            )
        };

        Ok(Response::new(register_reply))
    }
}