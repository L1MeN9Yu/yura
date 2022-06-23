use crate::rpc::base;
use user::account_server::{Account, AccountServer};

pub mod user {
    tonic::include_proto!("user");
}