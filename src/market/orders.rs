use std::fmt;

pub type Orders = Vec<Order>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Order {
    pub order_id: u64,
    pub system_id: i32,
    pub location_id: i32,
    pub is_buy_order: bool,
    pub type_id: i32,
    pub min_volume: i32,
    pub volume_remain: i32,
    pub volume_total: i32,
    pub price: f32,
    pub duration: i32,
    pub issued: String,
    pub range: String,
}
impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Order Id: {} Issued: {} Range: {}",
            self.order_id, self.issued, self.range
        )?;
        writeln!(f, "System Id: {}", self.system_id)?;
        writeln!(f, "Location Id: {}", self.location_id)?;
        writeln!(f, "Is buy order: {}", self.is_buy_order)?;
        writeln!(
            f,
            "{} {} {}/{} {} ISK {} days",
            self.type_id,
            self.min_volume,
            self.volume_remain,
            self.volume_total,
            self.price,
            self.duration
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use approx::assert_relative_eq;
    const EXAMPLE_VALUE: &str = r##"[
    {
        "duration": 90,
        "is_buy_order": false,
        "issued": "2016-09-03T05:12:25Z",
        "location_id": 60005599,
        "min_volume": 1,
        "order_id": 4623824223,
        "price": 9.9,
        "range": "region",
        "system_id": 30000053,
        "type_id": 34,
        "volume_remain": 1296000,
        "volume_total": 2000000
    }
    ]"##;

    #[test]
    fn parse() -> anyhow::Result<()> {

        let orders = serde_json::from_str::<Orders>(EXAMPLE_VALUE)?;
        assert_eq!(orders.len(), 1);
        let order = &orders[0];
        assert_eq!(order.duration, 90);
        assert_eq!(order.is_buy_order, false);
        assert_eq!(order.location_id, 60005599);
        assert_eq!(order.system_id, 30000053);
        assert_eq!(order.order_id, 4623824223);
        assert_eq!(order.type_id, 34);
        Ok(())
    }
}
