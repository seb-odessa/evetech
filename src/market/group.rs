use crate::universe::utils;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Group {
    pub market_group_id: i32,
    pub name: String,
    pub parent_group_id: Option<i32>,
    pub description: String,
    pub types: Vec<i32>,
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.market_group_id, self.name)?;
        if let Some(parent_id) = self.parent_group_id {
            writeln!(f, "Parent Group: {}", parent_id)?;
        }
        writeln!(f, "Description: {}", self.description)?;
        utils::write("Types", &Some(self.types.clone()), f)
    }
}
