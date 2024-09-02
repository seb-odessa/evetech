use crate::killmails;
use diesel::prelude::*;

use super::as_option;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::attackers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Attacker {
    pub killmail_id: i32,
    pub character_id: i32,
    pub corporation_id: i32,
    pub alliance_id: i32,
    pub faction_id: i32,
    pub damage_done: i32,
    pub final_blow: bool,
    pub security_status: f32,
    pub ship_type_id: i32,
    pub weapon_type_id: i32,
}
impl From<(u32, &killmails::Attacker)> for Attacker {
    fn from((id, attacker): (u32, &killmails::Attacker)) -> Self {
        Attacker {
            killmail_id: id as i32,
            character_id: attacker.character_id.unwrap_or_default() as i32,
            corporation_id: attacker.corporation_id.unwrap_or_default() as i32,
            alliance_id: attacker.alliance_id.unwrap_or_default() as i32,
            faction_id: attacker.faction_id.unwrap_or_default() as i32,
            damage_done: attacker.damage_done as i32,
            final_blow: attacker.final_blow,
            security_status: attacker.security_status,
            ship_type_id: attacker.ship_type_id.unwrap_or_default() as i32,
            weapon_type_id: attacker.weapon_type_id.unwrap_or_default() as i32,
        }
    }
}
impl Into<killmails::Attacker> for Attacker {
    fn into(self) -> killmails::Attacker {
        killmails::Attacker {
            character_id: as_option(self.character_id),
            corporation_id: as_option(self.corporation_id),
            alliance_id: as_option(self.alliance_id),
            faction_id: as_option(self.faction_id),
            damage_done: self.damage_done as u32,
            final_blow: self.final_blow,
            security_status: self.security_status,
            ship_type_id: as_option(self.ship_type_id),
            weapon_type_id: as_option(self.weapon_type_id),
        }
    }
}