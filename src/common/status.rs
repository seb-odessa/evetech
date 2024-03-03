#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Status {
    pub players: u32,
    pub server_version: String,
    pub start_time: String,
    pub vip: Option<bool>,
}
