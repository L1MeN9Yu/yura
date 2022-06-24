use log::{info};
use tonic::{Request, Response, Status};
use tonic::codegen::InterceptedService;
use rpc::account_server::{Account, AccountServer};
use rpc::{Reply, RegisterRequest, RegisterReply};

pub mod rpc {
    tonic::include_proto!("top.l1men9yu.yura");
}

#[derive(Debug, Default)]
pub struct RPCCenter {}

#[tonic::async_trait]
impl Account for RPCCenter {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterReply>, Status> {
        info!("Got a request: {:?}", request);

        let my_extension: Option<&MyExtension> = request.extensions().get();
        match my_extension {
            None => {} Some(my_) => {
                info!("{:?}",my_.some_piece_of_data);
            }
        }

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

    request.extensions_mut().insert(MyExtension {
        some_piece_of_data: "foo".to_string(),
    });

    Ok(request)
}

#[derive(Debug)]
struct MyExtension {
    some_piece_of_data: String,
}