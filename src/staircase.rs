#![allow(dead_code)]
/// Project: euclid
/// Module: staircase
///
/// This file contains geometric algorithms for computing the staircase of a
/// given set of points.
use crate::primitives::*;
use std::cmp::Ordering;

pub fn staircase(points: &Vec<Point<f32>>) -> Vec<&Point<f32>> {
    // sort the points in decreasing x-coordinate
    let mut points: Vec<&Point<f32>> = points.iter().map(|e| e).collect();
    points.sort_by(|a, b| match b.x().partial_cmp(&a.x()).unwrap() {
        Ordering::Equal => a.y().partial_cmp(&b.y()).unwrap(),
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    });
    let mut p_iter = points.iter();
    let mut stairpoints = vec![*p_iter.next().unwrap()];

    // keep track off the bar
    let mut floor = stairpoints.get(0).unwrap().y();
    while let Some(&p) = p_iter.next() {
        // no other points can be above and to the right of extremal points
        if p.y() > floor {
            stairpoints.push(p);
            // raise the floor
            floor = p.y();
        }
    }
    // reverse the list to produce the points from left to right
    stairpoints.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_1() {
        let input = vec![
            Point::from((0.0, 0.0)),
            Point::from((0.0, 5.0)),
            Point::from((1.0, 4.0)),
            Point::from((6.0, 2.0)),
            Point::from((2.0, 1.0)),
            Point::from((4.0, 0.0)),
            Point::from((-1.0, 3.0)),
        ];

        let output = staircase(&input);

        assert_eq!(
            output,
            vec![
                &Point::from((0.0, 5.0)),
                &Point::from((1.0, 4.0)),
                &Point::from((6.0, 2.0)),
            ]
        );
    }
}
