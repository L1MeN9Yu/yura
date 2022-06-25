pub(crate) mod table;

use lazy_static::lazy_static;
use rbatis;
use rbatis::rbatis::Rbatis;
use rbatis::Error;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

pub async fn init_database(url_string: String) -> Result<(), Error> {
    RB.link(url_string.as_str()).await
}

pub fn database() -> &'static Rbatis {
    &*RB
}
