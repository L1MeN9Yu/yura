#[crud_table]
#[derive(Clone, Debug)]
pub struct Account {
    pub id: Option<String>,
    pub name: String,
    pub password: String,
    pub token: String,
}