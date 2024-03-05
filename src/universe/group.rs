use crate::universe::utils;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Group {
    pub group_id: u32,
    pub name: String,
    pub category_id: u32,
    pub published: bool,
    pub types: Vec<u32>,
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.group_id, self.name)?;
        writeln!(f, "Published: {}", self.published)?;
        writeln!(f, "Category Id: {}", self.category_id)?;
        utils::write("Types", &Some(self.types.clone()), f)
    }
}
