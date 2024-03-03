use std::fmt;
use std::fmt::Display;


pub mod belt;
pub mod moon;
pub mod planet;
pub mod star;
pub mod stargate;
pub mod station;
pub mod system;

pub use belt::AsteroidBelts;
pub use moon::Moon;
pub use planet::Planet;
pub use star::Star;
pub use stargate::Stargate;
pub use stargate::StargateDestination;
pub use station::Station;
pub use system::PlanetSurrounding;
pub use system::System;

fn write<T>(title: &str, vec: &Option<Vec<T>>, f: &mut fmt::Formatter) -> fmt::Result
where
    T: Display + ToString,
{
    if let Some(ids) = &vec {
        let joined = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(f, "{title}: {}", joined)
    } else {
        write!(f, "")
    }
}