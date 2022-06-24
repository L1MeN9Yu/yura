use log::{info};
use rbatis::rbatis::Rbatis;
use tonic::{Request, Response, Status};
use tonic::codegen::InterceptedService;
use rpc::account_server::{Account, AccountServer};
use rpc::{Reply, RegisterRequest, RegisterReply};
use crate::database::database;
use super::super::database;

pub mod rpc {
    tonic::include_proto!("top.l1men9yu.yura");
}

#[derive(Debug, Default)]
pub struct RPCCenter {}

#[tonic::async_trait]
impl Account for RPCCenter {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterReply>, Status> {
        info!("Got a request: {:?}", request);

        let rb = database();

        info!("{:?}",rb);

        let register_reply = RegisterReply {
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

pub fn service() -> InterceptedService<AccountServer<RPCCenter>, fn(Request<()>) -> Result<Request<()>, Status>> {
    let rpc_center = RPCCenter::default();
    AccountServer::with_interceptor(rpc_center, intercept)
}

fn intercept(mut request: Request<()>) -> Result<Request<()>, Status> {
    info!("Intercepting request: {:?}", request);

    Ok(request)
}
