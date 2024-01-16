use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub user_id: u32,
    pub friend: Option<String>,
}