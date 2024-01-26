#![allow(dead_code)]
/// Project: euclid
/// Module: convex
/// 
/// This file contains geometric algorithms for computing the convex hull.

use crate::primitives::*;

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
