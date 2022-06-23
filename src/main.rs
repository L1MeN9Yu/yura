mod rpc;

use rpc::RPCCenter;
use rpc::rpc::account_server::AccountServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let rpc_center = RPCCenter::default();

    Server::builder()
        .add_service(AccountServer::new(rpc_center))
        .serve(addr)
        .await?;

    Ok(())
}