use std::fmt;


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Region {
    pub region_id: u32,
    pub name: String,
    pub description: Option<String>,
    pub constellations: Vec<u32>,
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.region_id, self.name)?;
        crate::universe::write("  Constellations", &Some(self.constellations.clone()), f)?;
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        write!(f, "")
    }
}