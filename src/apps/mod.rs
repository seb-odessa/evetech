use crate::common::Position;
use itertools::Itertools;
use log::{debug, warn};
use septem::Roman;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;

pub mod route;
pub mod waypoint;

pub use route::Route;
pub use waypoint::WayPoint;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Place {
    id: i32,
    name: String,
    position: Position,
    cloud_number: i32,
    belt_number: i32,
}
impl Place {
    pub fn new(id: &i32, name: &String, position: &Position) -> Self {
        let tokens = name.trim().split_whitespace().collect::<Vec<&str>>();
        assert_eq!(6, tokens.len());

        Self {
            id: id.clone(),
            name: name.clone(),
            position: position.clone(),
            cloud_number: *tokens[1].parse::<Roman>().unwrap() as i32,
            belt_number: tokens[5].parse::<i32>().unwrap_or_default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Cloud {
    places: HashMap<i32, Place>,
    distances: HashMap<i32, HashMap<i32, f64>>,
}
impl Cloud {
    pub fn new() -> Self {
        Self {
            places: HashMap::new(),
            distances: HashMap::new(),
        }
    }

    pub fn get_name(&self, id: &i32) -> Option<String> {
        if let Some(belt) = self.places.get(id) {
            Some(belt.name.clone())
        } else {
            None
        }
    }

    pub fn add(&mut self, id: &i32, name: &String, position: &Position) {
        let belt = Place::new(id, &name, &position);

        for (destination, belt) in &self.places {
            let distance = Position::distance(&position, &belt.position);
            debug!("Distance between {} and {} - {}", name, belt.name, distance);

            self.distances
                .entry(*id)
                .or_insert(HashMap::new())
                .insert(*destination, distance);

            self.distances
                .entry(*destination)
                .or_insert(HashMap::new())
                .insert(*id, distance);
        }

        if let Some(old) = self.places.insert(*id, belt) {
            warn!("The old value for {id} was replaced with: {:?}", old);
        }
    }

    pub fn distance_between(&self, a: &i32, b: &i32) -> Option<f64> {
        if let Some(ref value) = self.distances.get(a) {
            return value.get(b).cloned();
        }
        return None;
    }

    fn route_distance(&self, route: &Vec<&i32>) -> f64 {
        let mut distance = 0.0;
        route.iter().reduce(|a, b| {
            distance += self.distance_between(&a, &b).unwrap_or(0.0);
            return b;
        });
        return distance;
    }

    fn get_ids_sorted_by_name(&self) -> Vec<i32> {
        let mut places = self.places.values().cloned().collect::<Vec<Place>>();
        places.sort_by(|a, b| {
            if a.cloud_number == b.cloud_number {
                a.belt_number.cmp(&b.belt_number)
            } else {
                a.cloud_number.cmp(&b.cloud_number)
            }
        });
        places.into_iter().map(|belt| belt.id).collect::<Vec<i32>>()
    }

    pub fn get_ordinal_route(&self) -> (f64, Vec<i32>) {
        let points = self.get_ids_sorted_by_name();
        let refs = points.iter().collect::<Vec<&i32>>();
        (self.route_distance(&refs), points)
    }

    pub fn get_best_route(&self) -> (f64, Vec<i32>) {
        let points = self.get_ids_sorted_by_name();
        if points.is_empty() {
            (0.0, vec![])
        } else if 1 == points.len() {
            (0.0, points.clone())
        } else if 2 == points.len() {
            let refs = points.iter().collect::<Vec<&i32>>();
            (self.route_distance(&refs), points.clone())
        } else if points.len() < 10 {
            self.brute_force(&points)
        } else {
            self.lazzy_walker(&points)
        }
    }

    fn lazzy_walker(&self, points: &Vec<i32>) -> (f64, Vec<i32>) {
        let mut starts = LinkedList::new();
        for point in points {
            starts.push_back(point);
        }

        let mut min_dist = f64::MAX;
        let mut min_route = Vec::new();
        let mut count = points.len();
        while count > 0 {
            if let Some(point) = starts.pop_front() {
                let tail = starts.iter().cloned().cloned().collect::<Vec<i32>>();
                let (dist, route) = self.lazzy_walker_impl(vec![*point], tail);
                if dist < min_dist {
                    min_dist = dist;
                    min_route = route;
                }
                starts.push_back(point);
            }

            count -= 1;
        }

        return (min_dist, min_route);
    }

    fn lazzy_walker_impl(&self, mut route: Vec<i32>, mut points: Vec<i32>) -> (f64, Vec<i32>) {
        if points.is_empty() {
            let refs = route.iter().collect::<Vec<&i32>>();
            return (self.route_distance(&refs), route);
        }

        if let Some(point) = route.iter().last() {
            points.sort_by(|a, b| {
                let d_a = self.distance_between(point, a).unwrap();
                let d_b = self.distance_between(point, b).unwrap();
                d_b.partial_cmp(&d_a).unwrap()
            });
            if let Some(closest) = points.pop() {
                route.push(closest);
            }
        }
        return self.lazzy_walker_impl(route, points);
    }

    fn brute_force(&self, points: &Vec<i32>) -> (f64, Vec<i32>) {
        let mut minimal = f64::MAX;
        let mut route = Vec::new();
        let mut calculated = HashSet::new();
        for path in points.iter().permutations(points.len()) {
            if !calculated.contains(&path) {
                let mut reversed = path.clone();
                reversed.reverse();
                calculated.insert(reversed);

                let distance = self.route_distance(&path);
                if distance < minimal {
                    minimal = distance;
                    route = path.into_iter().cloned().collect();
                }
            }
        }

        return (minimal, route);
    }
}
