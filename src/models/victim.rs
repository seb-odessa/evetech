use crate::killmails;
use diesel::prelude::*;

use super::as_option;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::victims)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Victim {
    pub killmail_id: i32,
    pub character_id: i32,
    pub corporation_id: i32,
    pub alliance_id: i32,
    pub faction_id: i32,
    pub damage_taken: i32,
    pub ship_type_id: i32,
}
impl From<(i32, &killmails::Victim)> for Victim {
    fn from((id, victim): (i32, &killmails::Victim)) -> Self {
        Victim {
            killmail_id: id,
            character_id: victim.character_id.unwrap_or_default(),
            corporation_id: victim.corporation_id.unwrap_or_default(),
            alliance_id: victim.alliance_id.unwrap_or_default(),
            faction_id: victim.faction_id.unwrap_or_default(),
            damage_taken: victim.damage_taken,
            ship_type_id: victim.ship_type_id,
        }
    }
}

impl Into<killmails::Victim> for Victim {
    fn into(self) -> killmails::Victim {
        killmails::Victim {
            character_id: as_option(self.character_id),
            corporation_id: as_option(self.corporation_id),
            alliance_id: as_option(self.alliance_id),
            faction_id: as_option(self.faction_id),
            damage_taken: self.damage_taken,
            ship_type_id: self.ship_type_id,
            position: None,
            items: None
        }
    }
}
