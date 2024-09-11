#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Alliance {
    pub creator_corporation_id: i32,
    pub creator_id: i32,
    pub date_founded: String,
    pub executor_corporation_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub name: String,
    pub ticker: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str = r##"
    {
        "creator_corporation_id": 98241771,
        "creator_id": 379226154,
        "date_founded": "2013-08-23T05:45:11Z",
        "executor_corporation_id": 98688253,
        "name": "Fraternity.",
        "ticker": "FRT"
    }"##;

    #[test]
    fn parse() -> anyhow::Result<()> {
        let alliance = serde_json::from_str::<Alliance>(JSON)?;

        assert_eq!(alliance.creator_corporation_id, 98241771);
        assert_eq!(alliance.creator_id, 379226154);
        assert_eq!(&alliance.date_founded, "2013-08-23T05:45:11Z");
        assert_eq!(alliance.executor_corporation_id, Some(98688253));
        assert_eq!(alliance.faction_id, None);
        assert_eq!(&alliance.name, "Fraternity.");
        assert_eq!(&alliance.ticker, "FRT");
        Ok(())
    }
}
