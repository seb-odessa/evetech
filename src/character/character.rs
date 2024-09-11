#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Character {
    pub corporation_id: i32,
    pub alliance_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub birthday: String,
    pub bloodline_id: i32,
    pub race_id: i32,
    pub name: String,
    pub gender: String,
    pub description: Option<String>,
    pub security_status: f32,
    pub title: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const JSON: &str = r##"
    {
        "birthday": "2018-07-27T17:42:45Z",
        "bloodline_id": 1,
        "corporation_id": 98573194,
        "description": "",
        "gender": "male",
        "name": "Seb Odessa",
        "race_id": 1,
        "security_status": 5.002559608
    }"##;

    #[test]
    fn parse() -> anyhow::Result<()> {

        let character = serde_json::from_str::<Character>(JSON)?;

        assert_eq!(&character.birthday, "2018-07-27T17:42:45Z");
        assert_eq!(character.bloodline_id, 1);
        assert_eq!(character.corporation_id, 98573194);
        assert_eq!(character.description, Some(String::new()));
        assert_eq!(&character.gender, "male");
        assert_eq!(&character.name, "Seb Odessa");
        assert_eq!(character.race_id, 1);
        assert_relative_eq!(character.security_status, 5.0025597);

        Ok(())
    }
}