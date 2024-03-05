use std::fmt;
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Status {
    pub players: u32,
    pub server_version: String,
    pub start_time: String,
    pub vip: Option<bool>,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Players: {}, Version: {}, Startup: {})",
            self.players, self.server_version, self.start_time
        )
    }
}
