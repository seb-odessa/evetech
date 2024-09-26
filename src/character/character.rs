use crate::esi::api::Uid;
use crate::esi::api::Uri;
use crate::esi::CHARACTERS;
use crate::esi::PARAM;

use anyhow::anyhow;

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
impl Uri for Character {
    fn uri(id: &Uid) -> anyhow::Result<String> {
        if let Uid::Id(id) = id {
            Ok(format!("{CHARACTERS}/{id}/?{PARAM}"))
        } else {
            Err(anyhow!("Expected Uid::Id(i32)"))
        }
    }
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

    #[tokio::test]
    async fn load() -> anyhow::Result<()> {
        use crate::esi::EveApi;

        let api = EveApi::new();
        let obj = api.load::<Character>(&Uid::Id(2114350216)).await?;
        assert_eq!(&obj.birthday, "2018-07-27T17:42:45Z");
        assert_eq!(obj.bloodline_id, 1);
        assert_eq!(obj.corporation_id, 98573194);
        assert_eq!(obj.description, Some(String::new()));
        assert_eq!(&obj.gender, "male");
        assert_eq!(&obj.name, "Seb Odessa");
        assert_eq!(obj.race_id, 1);
        Ok(())
    }
}