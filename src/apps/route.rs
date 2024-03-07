use itertools::Itertools;

use crate::apps::waypoint::WayPoint;
use crate::common::Position;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

pub struct Route {
    pub start: WayPoint,
    pub belts: HashMap<u32, WayPoint>,
}
impl Route {
    pub fn new(start: WayPoint) -> Self {
        Self {
            start: start,
            belts: HashMap::new(),
        }
    }

    pub fn add(&mut self, point: WayPoint) {
        self.belts.insert(point.id, point);
    }

    pub fn brute_force(&self) -> (f64, Vec<u32>) {
        if self.belts.is_empty() {
            return (0.0, vec![self.start.id]);
        }

        let mut minimal = f64::MAX;
        let mut route = Vec::new();
        let mut calculated = HashSet::new();

        let ordinal = self.belts.keys().cloned().collect::<Vec<_>>();
        let len = ordinal.len();

        for path in ordinal.iter().permutations(len) {
            if !calculated.contains(&path) {
                let mut reversed = path.clone();
                reversed.reverse();
                // println!("reversed: {:?}", reversed);
                calculated.insert(reversed);


                let distance = self.length(&path);
                if distance < minimal {
                    minimal = distance;
                    route = path.into_iter().cloned().collect();
                    route.insert(0, self.start.id);
                }
            }
        }
        (minimal, route)
    }

    fn length(&self, route: &Vec<&u32>) -> f64 {
        let mut distance = 0.0;
        let mut previous = &self.start.position;
        let mut ids = route.iter();
        while let Some(id) = ids.next() {
            let current = &self
                .belts
                .get(id)
                .expect(&format!("Belt Id {id} must be in self.belts!"))
                .position;
            distance += self.distance(previous, current);
            previous = current;
        }
        return distance;
    }

    fn distance(&self, lhv: &Position, rhv: &Position) -> f64 {
        // NOTE May be used different algorithm, e.g. use requred angle
        //      instead of distance
        Position::distance(lhv, rhv)
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.start)?;
        for (_, wp) in &self.belts {
            writeln!(f, "{}", wp)?;
        }
        write!(f, "")
    }
}
