
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Attacker {
    pub character_id: Option<u32>,
    pub corporation_id: Option<u32>,
    pub alliance_id: Option<u32>,
    pub faction_id: Option<u32>,
    pub damage_done: u32,
    pub final_blow: bool,
    pub security_status: f32,
    pub ship_type_id: Option<u32>,
    pub weapon_type_id: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    const JSON: &str = r##"
    {
        "character_id": 3019582,
        "corporation_id": 1000274,
        "damage_done": 6804,
        "faction_id": 500024,
        "final_blow": true,
        "security_status": 0,
        "ship_type_id": 34495,
        "weapon_type_id": 34580
    }"##;

    #[test]
    fn parse() -> anyhow::Result<()> {
        let attacker = serde_json::from_str::<Attacker>(JSON)?;
        assert_eq!(attacker.character_id, Some(3019582));
        assert_eq!(attacker.corporation_id, Some(1000274));
        assert_eq!(attacker.alliance_id, None);
        assert_eq!(attacker.faction_id, Some(500024));
        assert_eq!(attacker.damage_done, 6804);
        assert_eq!(attacker.final_blow, true);
        assert_relative_eq!(attacker.security_status, 0.0);
        assert_eq!(attacker.ship_type_id, Some(34495));
        assert_eq!(attacker.weapon_type_id, Some(34580));
        Ok(())
    }
}
