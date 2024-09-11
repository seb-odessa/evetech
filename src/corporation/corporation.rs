#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Corporation {
    pub alliance_id: Option<i32>,
    pub ceo_id: i32,
    pub creator_id: i32,
    pub date_founded: Option<String>,
    pub description: Option<String>,
    pub faction_id: Option<i32>,
    pub home_station_id: Option<i32>,
    pub member_count: i32,
    pub name: String,
    pub shares: Option<i32>,
    pub tax_rate: f32,
    pub ticker: String,
    pub url: Option<String>,
    pub war_eligible: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const JSON: &str = r##"
    {
        "ceo_id": 2114350216,
        "creator_id": 2114350216,
        "date_founded": "2018-09-05T18:41:42Z",
        "description": "Description",
        "home_station_id": 60012112,
        "member_count": 7,
        "name": "SO Corporation",
        "shares": 1000,
        "tax_rate": 0.10000000149011612,
        "ticker": "SO C",
        "url": "http://vm1816097.firstbyte.club:8088/gui/who/"
    }"##;

    #[test]
    fn parse() -> anyhow::Result<()> {

        let corporation = serde_json::from_str::<Corporation>(JSON)?;

        assert_eq!(corporation.alliance_id, None);
        assert_eq!(corporation.ceo_id, 2114350216);
        assert_eq!(corporation.creator_id, 2114350216);
        assert_eq!(corporation.date_founded, Some(String::from("2018-09-05T18:41:42Z")));
        assert_eq!(corporation.description, Some(String::from("Description")));
        assert_eq!(corporation.faction_id, None);
        assert_eq!(corporation.home_station_id, Some(60012112));
        assert_eq!(corporation.member_count, 7);
        assert_eq!(&corporation.name, "SO Corporation");
        assert_eq!(corporation.shares, Some(1000));
        assert_relative_eq!(corporation.tax_rate, 0.10000000149);
        assert_eq!(&corporation.ticker, "SO C");
        assert_eq!(corporation.url, Some(String::from("http://vm1816097.firstbyte.club:8088/gui/who/")));
        assert_eq!(corporation.war_eligible, None);

        Ok(())
    }
}