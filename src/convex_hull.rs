#![allow(dead_code)]
/// Project: euclid
/// Module: convex_hull
/// 
/// This file contains geometric algorithms for computing the convex hull.

use crate::primitives::*;
use std::cmp::Ordering;

/// Computes the convex hull for a set of 2-dimensional `points`.
/// 
/// The output includes collinear points along the convex polygon boundary. The
/// set of points to form the convex polygon are sorted in counter-clockwise
/// order.
/// 
/// - Time complexity: O(n^3)
pub fn brute_force(points: &Vec<Point>) -> Vec<&Point> {

    let mut edges: Vec<(&Point, &Point, f32)> = Vec::new();
    // search through every possible pair of points
    for p in points {
        for q in points.iter().filter(|q| q != &p) {
            // check if all other points lie on the left of this directed line segment
            let mut valid = true;
            for r in points.iter().filter(|r| r != &p && r != &q) {
                // allow points that are collinear or to the left of the directed line segment
                if direction(p, q, r) == Orientation::Right {
                    valid = false;
                    break;
                }
            }
            if valid == true {
                edges.push((p, q, euclid_dist(p, q)));
            }
        }
    }

    // create a list of the points in order from the edges to form a polygon
    let mut polygon = Vec::new();

    let mut e = edges.get(0).unwrap();
    loop {
        polygon.push(e.0);
        let mut next = e;
        // pick the edge with the shortest distance
        for n in &edges {
            if e.0 == n.0 && n.2 < next.2 {
                next = n;
            }
        }
        // move to next link in the directed edges
        for n in &edges {
            if next.1 == n.0 {
                e = n;
                break;
            }
        }
        // looped back to the initial point (polygon is complete)
        if &e.0 == polygon.get(0).unwrap() {
            break;
        }
    }

    polygon
}


/// Computes the convex hull for a set of 2-dimensional `points`.
/// 
/// The output includes collinear points along the convex polygon boundary. The
/// set of points to form the convex polygon are sorted in counter-clockwise
/// order.
/// 
/// - Time complexity: O(n*log(n))
pub fn upper_lower(points: &Vec<Point>) -> Vec<&Point> {
    // sort the points by increasing x-coordinate, and then increasing y-coordinate (if equal x-coordinate)
    let mut points: Vec<&Point> = points.iter().map(|e| e).collect();
    points.sort_by(|a, b| match a.x().partial_cmp(&b.x()).unwrap() {
        Ordering::Equal => a.y().partial_cmp(&b.y()).unwrap(),
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    });

    // construct the upper hull
    let mut p_iter = points.iter();
    let mut upper: Vec<&Point> = vec![p_iter.next().unwrap(), p_iter.next().unwrap()];
    // references to the previous two points in the polygon
    let mut q = *upper.get(upper.len()-1).unwrap();
    let mut p = *upper.get(upper.len()-2).unwrap();
    // incrementally add points from left to right only maintaining right turns
    while let Some(&r) = p_iter.next() {
        if r == q { continue }
        while upper.len() > 1 && direction(p, q, r) == Orientation::Left {
            // delete the middle of the last three points
            upper.pop();
            if upper.len() == 1 {
                break;
            }
            q = *upper.get(upper.len()-1).unwrap();
            p = *upper.get(upper.len()-2).unwrap();
        }
        upper.push(r);
        // update references to the previous 2 points in the hull
        q = *upper.get(upper.len()-1).unwrap();
        p = *upper.get(upper.len()-2).unwrap();
    }

    // construct the lower hull
    let mut p_iter = points.iter().rev();
    let mut lower: Vec<&Point> = vec![p_iter.next().unwrap(), p_iter.next().unwrap()];
    // references to the previous two points in the polygon
    let mut q = *lower.get(lower.len()-1).unwrap();
    let mut p = *lower.get(lower.len()-2).unwrap();
    // incrementally add points from right to left only maintaining right turns
    while let Some(&r) = p_iter.next() {
        if r == q { continue }
        while lower.len() > 1 && direction(p, q, r) == Orientation::Left {
            // delete the middle of the last three points
            lower.pop();
            if lower.len() == 1 {
                break;
            }
            q = *lower.get(lower.len()-1).unwrap();  
            p = *lower.get(lower.len()-2).unwrap();
        }
        lower.push(r);        
        // update references to the previous 2 points in the hull
        q = *lower.get(lower.len()-1).unwrap();
        p = *lower.get(lower.len()-2).unwrap();
    }

    // combine the two hulls and remove duplicate endpoints while in ccw order
    upper.into_iter().chain(lower.into_iter().skip(1).rev().skip(1).rev()).rev().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_upper_lower_edge_case_1() {
        let points = crate::read_points("tests/convex_hull/ec1.txt");
        let hull = upper_lower(&points);
        assert_eq!(hull.len(), 33);
    }

    #[test]
    fn ut_upper_lower_edge_case_2() {
        let points = crate::read_points("tests/convex_hull/in1.txt");
        let hull = upper_lower(&points);
        assert_eq!(hull.len(), 33);
    }

    #[test]
    fn ut_brute_force_edge_case_1() {
        let points = crate::read_points("tests/convex_hull/ec1.txt");
        let hull = brute_force(&points);
        assert_eq!(hull.len(), 33);
    }

    #[test]
    fn ut_upper_lower_normal() {
        let input = vec![
            Point::from((0.0, 0.0)),
            Point::from((2.0, 10.0)),
            Point::from((5.0, 1.0)),
            Point::from((2.0, 4.0)),
            Point::from((3.0, 2.0)),
        ];

        let output = upper_lower(&input);

        assert_eq!(output,
            vec![
                &Point::from((5.0, 1.0)), 
                &Point::from((2.0, 10.0)),
                &Point::from((0.0, 0.0)), 
            ]
        );
    }

    #[test]
    fn ut_upper_lower_collinear() {
        let input = vec![
            Point::from((0.0, 0.0)),
            Point::from((6.0, 0.0)),
            Point::from((5.0, 5.0)),
            Point::from((3.0, 0.0)),
            Point::from((4.0, 0.0)),
            Point::from((1.0, 5.0)),
            Point::from((2.0, 2.0)),
        ];

        let output = upper_lower(&input);

        assert_eq!(output,
            vec![
                &Point::from((3.0, 0.0)), 
                &Point::from((4.0, 0.0)), 
                &Point::from((6.0, 0.0)), 
                &Point::from((5.0, 5.0)), 
                &Point::from((1.0, 5.0)),
                &Point::from((0.0, 0.0)), 
            ]
        );
    }

    #[test]
    fn ut_brute_force_normal() {
        let input = vec![
            Point::from((0.0, 0.0)),
            Point::from((2.0, 10.0)),
            Point::from((5.0, 1.0)),
            Point::from((2.0, 4.0)),
            Point::from((3.0, 2.0)),
        ];

        let output = brute_force(&input);

        assert_eq!(output,
            vec![
                &Point::from((0.0, 0.0)), 
                &Point::from((5.0, 1.0)), 
                &Point::from((2.0, 10.0))
            ]
        );
    }

    #[test]
    fn ut_brute_force_collinear() {
        let input = vec![
            Point::from((0.0, 0.0)),
            Point::from((6.0, 0.0)),
            Point::from((5.0, 5.0)),
            Point::from((3.0, 0.0)),
            Point::from((4.0, 0.0)),
            Point::from((1.0, 5.0)),
            Point::from((2.0, 2.0)),
        ];

        let output = brute_force(&input);

        assert_eq!(output,
            vec![
                &Point::from((0.0, 0.0)), 
                &Point::from((3.0, 0.0)), 
                &Point::from((4.0, 0.0)), 
                &Point::from((6.0, 0.0)), 
                &Point::from((5.0, 5.0)), 
                &Point::from((1.0, 5.0))
            ]
        );
    }

}