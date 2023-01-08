use std::collections::HashSet;

use crate::files;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Deployment {
    sensor_location: Point,
    closest_beacon_location: Point,
}

impl Deployment {
    pub fn from_str(str: &String) -> Self {
        let mut locations = str.splitn(2, ":");
        let sensor_location = Deployment::extract_point(locations.next().unwrap());
        let closest_beacon_location = Deployment::extract_point(locations.next().unwrap());
        Self { sensor_location, closest_beacon_location }
    }

    fn extract_point(string: &str) -> Point {
        let splits: Vec<_> = string.splitn(4, |c| c == '=' || c == ',').collect();
        let x = splits[1].parse::<i32>().unwrap();
        let y = splits[3].parse::<i32>().unwrap();
        Point { x, y }
    }

    pub fn covers(&self, line: i32) -> bool {
        let sensor = self.sensor_location;
        let distance = self.sensor_beacon_distance();
        line >= sensor.y - distance && line <= sensor.y + distance
    }

    fn sensor_beacon_distance(&self) -> i32 {
        let p1 = self.sensor_location;
        let p2 = self.closest_beacon_location;
        (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
    }

    pub fn covered_points_in(&self, line: &i32, min: &i32, max: &i32) -> Vec<i32> {
        let sensor = self.sensor_location;
        let sensor_beacon_distance = self.sensor_beacon_distance();
        let delta_vertical = (sensor.y - line).abs();
        let delta_horizontal = sensor_beacon_distance - delta_vertical;
        let start = *cmp::max(min, &(sensor.x - delta_horizontal));
        let end = *cmp::min(max, &(sensor.x + delta_horizontal));
        (start..=end).collect()
    }
}


pub fn solve() {
    solve_part1()
}


fn solve_part1() {
    let file = "resources/day15.txt";
    let lines = files::parse_lines_from(file);

    let requested_line = 2_000_000;
    let min = i32::MIN;
    let max = i32::MAX;
    let deployments = lines
        .iter()
        .map(|line| Deployment::from_str(line))
        .collect::<Vec<Deployment>>();

    let mut covered_xs_in_requested_line = deployments.iter()
        .filter(|deployment| deployment.covers(requested_line))
        .flat_map(|deployment| deployment.covered_points_in(&requested_line, &min, &max))
        .collect::<HashSet<i32>>();

    for deployment in deployments {
        if deployment.sensor_location.y == requested_line {
            covered_xs_in_requested_line.remove(&deployment.sensor_location.x);
        }
        if deployment.closest_beacon_location.y == requested_line {
            covered_xs_in_requested_line.remove(&deployment.closest_beacon_location.x);
        }
    }

    println!("Part 1: {}", covered_xs_in_requested_line.len())
}
