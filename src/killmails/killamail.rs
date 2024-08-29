use std::fmt;

use super::{attacker::Attaker, victim::Victim};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Killmail {
    pub killmail_id: u32,
    pub killmail_time: String,
    pub solar_system_id: u32,
    pub moon_id: Option<u32>,
    pub war_id: Option<u32>,
    pub attackers: Vec<Attaker>,
    pub victim: Victim,
}
impl fmt::Display for Killmail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "killmail_id: {}", self.killmail_id)?;
        writeln!(f, "killmail_time: {}", self.killmail_time)?;
        writeln!(f, "solar_system_id: {}", self.solar_system_id)?;
        if let Some(moon_id) = self.moon_id {
            writeln!(f, "moon_id: {}", moon_id)?;
        }
        if let Some(war_id) = self.war_id {
            writeln!(f, "war_id: {}", war_id)?;
        }
        writeln!(f, "attakers: ")?;
        for attacker in &self.attackers {
            writeln!(f, "{}", attacker)?;
        }
        writeln!(f, "victim: {}", self.victim)
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::Position, killmails::item::Item};

    use super::*;
    const JSON: &str = r##"
    {
        "attackers": [
            {
                "character_id": 3019582,
                "corporation_id": 1000274,
                "damage_done": 6804,
                "faction_id": 500024,
                "final_blow": true,
                "security_status": 0,
                "ship_type_id": 34495,
                "weapon_type_id": 34580
            },
            {
                "character_id": 3019581,
                "corporation_id": 1000274,
                "damage_done": 4538,
                "faction_id": 500024,
                "final_blow": false,
                "security_status": 0,
                "ship_type_id": 34495,
                "weapon_type_id": 34580
            },
            {
                "damage_done": 0,
                "faction_id": 500024,
                "final_blow": false,
                "security_status": 0,
                "ship_type_id": 34495
            }
        ],
        "killmail_id": 120461567,
        "killmail_time": "2024-08-27T03:54:10Z",
        "solar_system_id": 30004563,
        "victim": {
            "alliance_id": 1900696668,
            "character_id": 2120326223,
            "corporation_id": 98316235,
            "damage_taken": 11342,
            "items": [
                {
                    "flag": 93,
                    "item_type_id": 31724,
                    "quantity_destroyed": 1,
                    "singleton": 0
                },
                {
                    "flag": 21,
                    "item_type_id": 3831,
                    "quantity_dropped": 1,
                    "singleton": 0
                }
            ],
            "position": {
                "x": -955007564796.388,
                "y": -126124010916.492,
                "z": 726635633538.084
            },
            "ship_type_id": 81008
        }
    }"##;

    #[test]
    fn parse() -> anyhow::Result<()> {
        let killmail = serde_json::from_str::<Killmail>(JSON)?;
        assert_eq!(killmail.killmail_id, 120461567);
        assert_eq!(killmail.killmail_time, "2024-08-27T03:54:10Z".to_owned());
        assert_eq!(killmail.solar_system_id, 30004563);
        assert_eq!(
            killmail.attackers,
            vec![
                Attaker {
                    character_id: Some(3019582),
                    corporation_id: Some(1000274),
                    alliance_id: None,
                    faction_id: Some(500024),
                    damage_done: 6804,
                    final_blow: true,
                    security_status: 0.0,
                    ship_type_id: Some(34495),
                    weapon_type_id: Some(34580)
                },
                Attaker {
                    character_id: Some(3019581),
                    corporation_id: Some(1000274),
                    alliance_id: None,
                    faction_id: Some(500024),
                    damage_done: 4538,
                    final_blow: false,
                    security_status: 0.0,
                    ship_type_id: Some(34495),
                    weapon_type_id: Some(34580)
                },
                Attaker {
                    character_id: None,
                    corporation_id: None,
                    alliance_id: None,
                    faction_id: Some(500024),
                    damage_done: 0,
                    final_blow: false,
                    security_status: 0.0,
                    ship_type_id: Some(34495),
                    weapon_type_id: None
                }
            ]
        );
        assert_eq!(
            killmail.victim,
            Victim {
                character_id: Some(2120326223),
                corporation_id: Some(98316235),
                alliance_id: Some(1900696668),
                faction_id: None,
                damage_taken: 11342,
                position: Some(Position {
                    x: -955007564796.388,
                    y: -126124010916.492,
                    z: 726635633538.084
                }),
                ship_type_id: 81008,
                items: Some(vec![
                    Item {
                        flag: 93,
                        item_type_id: 31724,
                        quantity_destroyed: Some(1),
                        quantity_dropped: None,
                        singleton: 0
                    },
                    Item {
                        flag: 21,
                        item_type_id: 3831,
                        quantity_destroyed: None,
                        quantity_dropped: Some(1),
                        singleton: 0
                    }
                ])
            }
        );
        Ok(())
    }
}
