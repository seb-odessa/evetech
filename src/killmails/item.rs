
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Item {
    pub flag: i32,
    pub item_type_id: i32,
    pub quantity_destroyed: Option<u64>,
    pub quantity_dropped: Option<u64>,
    pub singleton: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    const JSON: &str = r##"
        {
            "flag": 93,
            "item_type_id": 31724,
            "quantity_destroyed": 1,
            "singleton": 0
        }"##;

    #[test]
    fn parse() -> anyhow::Result<()> {
        let item = serde_json::from_str::<Item>(JSON)?;
        assert_eq!(item.flag, 93);
        assert_eq!(item.item_type_id, 31724);
        assert_eq!(item.quantity_destroyed, Some(1));
        assert_eq!(item.quantity_dropped, None);
        assert_eq!(item.singleton, 0);
        Ok(())
    }
}
