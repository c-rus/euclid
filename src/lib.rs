#![allow(dead_code)]
/// Library: euclid
/// 
/// This file contains geometric primitives for performing computations in
/// 2-dimensional space.

pub mod primitives;
pub mod convex_hull;
pub mod staircase;

use primitives::Point;

/// Opens and reads the file at `path` to parse its contents into a list
/// of `Point` structures.
pub fn read_points(path: &str) -> Vec<Point> {
    let contents = std::fs::read_to_string(path).unwrap();

    contents.split('\n').filter(|s| s.len() > 0).map(|s| {
        let pair = s.split_once(' ').unwrap();
        let pair: (f32, f32) = (pair.0.parse().unwrap(), pair.1.parse().unwrap());
        Point::from(pair)
    }).collect()
}

/// Creates a file and writes a list of `Point` structures as the contents.
pub fn write_points(path: &str, points: Vec<&Point>) -> () {
    let contents: Vec<String> = points.iter().map(|f| {
        format!("{} {}\n", f.x(), f.y())
    }).collect();
    let contents: String = contents.iter().flat_map(|s| s.chars()).collect();

    std::fs::write(path, contents.as_bytes()).unwrap()
}