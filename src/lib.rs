#[macro_use]
extern crate serde_derive;

// extern crate serde;
// #[macro_use]
// extern crate serde_json;


pub mod openapi;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
