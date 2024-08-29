use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Item {
    pub flag: u32,
    pub item_type_id: u32,
    pub quantity_destroyed: Option<u64>,
    pub quantity_dropped: Option<u64>,
    pub singleton: u32,
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "flag: {}", self.flag)?;
        writeln!(f, "item_type_id: {}", self.item_type_id)?;
        if let Some(quantity_destroyed) = self.quantity_destroyed {
            writeln!(f, "quantity_destroyed: {}", quantity_destroyed)?;
        }
        if let Some(quantity_dropped) = self.quantity_dropped {
            writeln!(f, "quantity_dropped: {}", quantity_dropped)?;
        }
        writeln!(f, "singleton: {}", self.singleton)
    }
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
