use crate::common::Position;
use super::item::Item;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Victim {
    pub character_id: Option<i32>,
    pub corporation_id: Option<i32>,
    pub alliance_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub damage_taken: i32,
    pub position: Option<Position>,
    pub ship_type_id: i32,
    pub items: Option<Vec<Item>>,
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
