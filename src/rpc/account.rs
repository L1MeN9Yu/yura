use super::super::database::*;
use log::info;
use rbatis::crud::CRUD;
use rpc::account_server::{Account, AccountServer};
use rpc::{RegisterReply, RegisterRequest, Reply};
use tonic::{Request, Response, Status};

pub mod rpc {
    tonic::include_proto!("top.l1men9yu.yura");
}

#[derive(Debug, Default)]
pub struct RPCCenter {}

#[tonic::async_trait]
impl Account for RPCCenter {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterReply>, Status> {
        info!("Got a request: {:?}", request);

        let register_request = request.into_inner();
        let name = register_request.name;
        let password = register_request.password;

        let rb = database();
        let account = table::account::Account {
            id: None,
            name,
            password,
            token: "123123".to_string(),
        };

        rb.save(&account, &[]).await.map_err(|error| super::status::from(error))?;

        info!("{:?}", rb);

        let register_reply = RegisterReply {
            reply: Some(Reply { code: 0, message: "success".to_string() }),
        };

        Ok(Response::new(register_reply))
    }
}

pub fn service() -> AccountServer<RPCCenter> {
    let rpc_center = RPCCenter::default();
    AccountServer::new(rpc_center)
}
