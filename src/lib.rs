#![allow(dead_code)]
/// Library: euclid
///
/// This file contains geometric primitives for performing computations in
/// 2-dimensional space.

pub mod convex_hull;
pub mod kd_tree;
pub mod primitives;
pub mod range_tree;
pub mod staircase;
pub mod point_loc;

use primitives::Point;
use primitives::LineSegment;
use std::fmt::Debug;
use std::str::FromStr;

/// Opens and reads the file at `path` to parse its contents into a list
/// of `Point` structures.
pub fn read_points<T: std::str::FromStr + std::default::Default + Copy>(path: &str) -> Vec<Point<T>>
where
    <T as FromStr>::Err: Debug,
{
    let contents = std::fs::read_to_string(path).unwrap();

    contents
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| {
            let pair = s.split_once(' ').unwrap();
            let pair: (T, T) = (pair.0.parse().unwrap(), pair.1.parse().unwrap());
            Point::from(pair)
        })
        .collect()
}

/// Creates a file and writes a list of `Point` structures as the contents.
pub fn write_points<T: std::default::Default + std::fmt::Display + Copy>(
    path: &str,
    points: Vec<&Point<T>>,
) -> () {
    let contents: Vec<String> = points
        .iter()
        .map(|f| format!("{} {}\n", f.x().to_string(), f.y().to_string()))
        .collect();
    let contents: String = contents.iter().flat_map(|s| s.chars()).collect();

    std::fs::write(path, contents.as_bytes()).unwrap()
}

/// Opens and reads the file at `path` to parse its contents into a list
/// of `Point` structures.
pub fn read_segments(path: &str) -> Vec<LineSegment> {
    let contents = std::fs::read_to_string(path).unwrap();

    contents
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| {
            let pair = s.split_once(',').unwrap();
            let p = pair.0.split_once(' ').unwrap();
            let q = pair.1.split_once(' ').unwrap();
            LineSegment::from((
                Point::from((p.0.parse().unwrap(), p.1.parse().unwrap())),
                Point::from((q.0.parse().unwrap(), q.1.parse().unwrap())),
            ))
        })
        .collect()
}
