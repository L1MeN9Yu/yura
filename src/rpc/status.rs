use rbatis;
use tonic::Status;

pub fn from(error: rbatis::Error) -> Status {
    return match error {
        rbatis::Error::E(message) => Status::internal(message),
        rbatis::Error::Deserialize(message) => Status::invalid_argument(message),
        rbatis::Error::Database(message) => Status::already_exists(message),
        _ => Status::unknown(""),
    };
}
