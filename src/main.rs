mod rpc;
mod config;
mod logger;
mod database;

#[macro_use]
extern crate rbatis;

use log::info;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = config::load_config()?;
    logger::init_logger(configuration.logs_directory)?;

    database::init_database(configuration.database.url).await?;

    let listen_address = configuration.listen_address.parse()?;

    info!("server start {:?}",listen_address);

    Server::builder()
        .add_service(rpc::account::service())
        .serve(listen_address)
        .await?;

    Ok(())
}
