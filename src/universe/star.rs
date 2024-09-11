use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
pub struct Star {
    pub name: String,
    pub age: u64,
    pub spectral_class: String,
    pub temperature: i32,
    pub radius: i64,
    pub solar_system_id: i32,
    pub type_id: i32,
}
impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Age: {}", self.age)?;
        writeln!(f, "Spectral Class: {}", self.spectral_class)?;
        writeln!(f, "Temperature: {}", self.temperature)?;
        writeln!(f, "Radius: {}", self.radius)?;
        writeln!(f, "type_id: {}", self.type_id)?;
        writeln!(f, "System Id: {}", self.solar_system_id)
    }
}
