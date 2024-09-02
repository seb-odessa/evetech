use crate::killmails;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::killmails)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Killmail {
    pub killmail_id: i32,
    pub killmail_time: String,
    pub solar_system_id: i32,
    pub moon_id: Option<i32>,
    pub war_id: Option<i32>,
}

impl From<&killmails::Killmail> for Killmail {
    fn from(killmail: &killmails::Killmail) -> Self {
        Killmail {
            killmail_id: killmail.killmail_id as i32,
            killmail_time: killmail.killmail_time.clone(),
            solar_system_id: killmail.solar_system_id as i32,
            moon_id: killmail.moon_id.map(|x| x.try_into().ok()).flatten(),
            war_id: killmail.war_id.map(|x| x.try_into().ok()).flatten(),
        }
    }
}