use std::fmt;
use std::fmt::Display;

pub fn write<T>(title: &str, vec: &Option<Vec<T>>, f: &mut fmt::Formatter) -> fmt::Result
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
