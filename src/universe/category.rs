use crate::universe;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Category {
    pub category_id: u32,
    pub name: String,
    pub published: bool,
    pub groups: Vec<u32>,
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.category_id, self.name)?;
        writeln!(f, "Published: {}", self.published)?;
        universe::write("Types", &Some(self.groups.clone()), f)
    }
}
