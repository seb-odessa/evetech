use itertools::Itertools;

use crate::apps::waypoint::WayPoint;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct Route {
    start: WayPoint,
    waypoints: HashMap<i32, WayPoint>,
    order: Vec<i32>,
}
impl Route {
    pub fn new(start: WayPoint) -> Self {
        Self {
            start: start,
            waypoints: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn add(&mut self, point: WayPoint) {
        let id = point.id;
        self.waypoints.insert(id, point);
        self.order.push(id);
    }

    pub fn start(&self) -> &WayPoint {
        &self.start
    }

    pub fn set_departue(&mut self, start: WayPoint) {
        self.start = start;
    }

    pub fn get(&self, id: &i32) -> Option<&WayPoint> {
        if self.start.id.eq(id) {
            return Some(&self.start);
        }
        self.waypoints.get(id)
    }

    pub fn order(&self) -> &Vec<i32> {
        &self.order
    }

    pub fn complete(&self) -> Vec<i32> {
        std::iter::once(self.start.id)
            .chain(self.order.iter().cloned())
            .collect()
    }

    pub fn len(&self) -> f64 {
        self.route_length(&self.order)
    }

    pub fn build_best(&mut self) {
        if self.order().len() < 10 {
            self.find_route_bruteforce()
        } else {
            self.find_route_greedy()
        }
    }

    fn route_length(&self, route: &Vec<i32>) -> f64 {
        let mut distance = 0.0;
        let mut current: Option<&WayPoint> = None;
        for id in route {
            let waipoint: &WayPoint = self
                .waypoints
                .get(id)
                .expect(&format!("The {id} must be in map"));
            if let Some(current) = current {
                distance += current.distance_to(&waipoint);
            }
            current = Some(&waipoint);
        }
        return distance;
    }

    pub fn find_route_bruteforce(&mut self) {
        let mut minimal = f64::MAX;
        let permutations = self.order.iter().permutations(self.order.len());
        let mut best = Vec::new();
        for permutaion in permutations {
            let mut route = permutaion.into_iter().cloned().collect();
            let length = self.route_length(&route);
            if length < minimal {
                minimal = length;
                swap(&mut best, &mut route);
            }
        }
        swap(&mut self.order, &mut best);
    }

    pub fn find_route_greedy(&mut self) {
        let mut visited: HashSet<i32> = HashSet::new();
        let mut route: Vec<i32> = Vec::new();
        if let Some(id) = self.closest(&self.start, &visited) {
            let mut current = self
                .waypoints
                .get(&id)
                .expect(&format!("The {id} must be in map"));

            visited.insert(id);
            route.push(id);

            while visited.len() < self.waypoints.len() {
                if let Some(next_id) = self.closest(current, &visited) {
                    visited.insert(next_id);
                    route.push(next_id);
                    current = self
                        .waypoints
                        .get(&next_id)
                        .expect(&format!("The {id} must be in map"));
                } else {
                    break;
                }
            }
        }
        swap(&mut self.order, &mut route);
    }

    fn closest(&self, current: &WayPoint, visited: &HashSet<i32>) -> Option<i32> {
        let mut closest: Option<&WayPoint> = None;
        let mut min_distance = f64::MAX;

        for waypoint in self.waypoints.values() {
            if current.id != waypoint.id {
                if !visited.contains(&waypoint.id) {
                    let distance = current.distance_to(waypoint);
                    if distance < min_distance {
                        min_distance = distance;
                        closest = Some(waypoint);
                    }
                }
            }
        }
        closest.map(|p| p.id)
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.start)?;
        for (_, wp) in &self.waypoints {
            writeln!(f, "{}", wp)?;
        }
        write!(f, "")
    }
}
