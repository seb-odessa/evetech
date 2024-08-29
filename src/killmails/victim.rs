use crate::common::Position;
use std::fmt;

use super::item::Item;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Victim {
    pub character_id: Option<u32>,
    pub corporation_id: Option<u32>,
    pub alliance_id: Option<u32>,
    pub faction_id: Option<u32>,
    pub damage_taken: u32,
    pub position: Option<Position>,
    pub ship_type_id: u32,
    pub items: Option<Vec<Item>>,
}

impl fmt::Display for Victim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(character_id) = self.character_id {
            writeln!(f, "character_id: {}", character_id)?;
        }
        if let Some(corporation_id) = self.corporation_id {
            writeln!(f, "corporation_id: {}", corporation_id)?;
        }
        if let Some(alliance_id) = self.alliance_id {
            writeln!(f, "alliance_id: {}", alliance_id)?;
        }
        if let Some(faction_id) = self.faction_id {
            writeln!(f, "faction_id: {}", faction_id)?;
        }
        writeln!(f, "damage_taken: {}", self.damage_taken)?;
        if let Some(position) = &self.position {
            writeln!(f, "position: {}", position)?;
        }
        writeln!(f, "ship_type_id: {}", self.ship_type_id)?;
        if let Some(items) = &self.items {
            for item in items {
                writeln!(f, "item: {}", item)?;
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const JSON: &str = r##"
    {
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
                "flag": 185,
                "item_type_id": 81144,
                "quantity_destroyed": 1924,
                "singleton": 0
            }
        ],
        "position": {
            "x": -955007564796.388,
            "y": -126124010916.492,
            "z": 726635633538.084
        },
        "ship_type_id": 81008
    }"##;

    #[test]
    fn parse() -> anyhow::Result<()> {
        let victim = serde_json::from_str::<Victim>(JSON)?;
        assert_eq!(victim.character_id, Some(2120326223));
        assert_eq!(victim.corporation_id, Some(98316235));
        assert_eq!(victim.alliance_id, Some(1900696668));
        assert_eq!(victim.faction_id, None);
        assert_eq!(victim.damage_taken, 11342);
        assert_eq!(victim.ship_type_id, 81008);
        assert_eq!(
            victim.items,
            Some(vec![
                Item {
                    flag: 93,
                    item_type_id: 31724,
                    quantity_destroyed: Some(1),
                    quantity_dropped: None,
                    singleton: 0
                },
                Item {
                    flag: 185,
                    item_type_id: 81144,
                    quantity_destroyed: Some(1924),
                    quantity_dropped: None,
                    singleton: 0
                }
            ])
        );
        Ok(())
    }
}
