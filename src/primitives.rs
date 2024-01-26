#![allow(dead_code)]
/// Project: euclid
/// Module: primitives
/// 
/// This file contains geometric primitives for performing computations in
/// 2-dimensional space.

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Left,
    Right,
    Straight,
}

/// A numerical value that defines a singular dimension.
pub type Coordinate = f32;

/// A location, without size, in 2-dimensional space.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point(Coordinate, Coordinate);


impl From<(f32, f32)> for Point {
    fn from(pair: (f32, f32)) -> Self { 
        Self(pair.0, pair.1)
    }
}

impl Point {
    /// Create a new point at the origin (0, 0).
    pub fn new() -> Self {
        Self(0.0, 0.0)
    }

    /// Returns the coordinate found on the x-axis.
    pub fn x(&self) -> Coordinate {
        self.0
    }

    /// Returns the coordinate found on the y-axis.
    pub fn y(&self) -> Coordinate {
        self.1
    }
}

/// A line consisting of a start point and an end point.
pub struct LineSegment(Point, Point);

impl LineSegment {
    /// Returns the minimum value across the x-axis. 
    pub fn x_min(&self) -> Coordinate {
        if &self.0.x() < &self.1.x() { self.0.x() } else { self.1.x() }
    }

    /// Returns the maximum value across the x-axis.
    pub fn x_max(&self) -> Coordinate {
        if &self.0.x() > &self.1.x() { self.0.x() } else { self.1.x() }
    }

    /// Returns the minimum value across the y-axis. 
    pub fn y_min(&self) -> Coordinate {
        if &self.0.y() < &self.1.y() { self.0.y() } else { self.1.y() }
    }

    /// Returns the maximum value across the y-axis.
    pub fn y_max(&self) -> Coordinate {
        if &self.0.y() > &self.1.y() { self.0.y() } else { self.1.y() }
    }

    /// Returns the first point in the line segment.
    pub fn start(&self) -> &Point {
        &self.0
    }

    /// Returns the terminating point in the line segment.
    pub fn end(&self) -> &Point {
        &self.1
    }

}

/// Computes the cross-product among the set of points.
/// 
/// Equation: (`p1` - `p0`) x (`p2` - `p0`)
pub fn cross_prod(p0: &Point, p1: &Point, p2: &Point) -> f32 {
    (p1.x() - p0.x()) * (p2.y() - p0.y()) - (p2.x() - p0.x()) * (p1.y() - p0.y())
}

/// Computes the relative orientation using the cross-product method.
/// 
/// Given two line segments p0p1 and p1p2, if we traverse p0p1 and then p1p2, do
/// we make a left turn at point p1?
pub fn direction(p0: &Point, p1: &Point, p2: &Point) -> Orientation {
    let cp = cross_prod(&p0, &p2, &p1);
    if cp > 0.0 {
        Orientation::Right
    } else if cp < 0.0 {
        Orientation::Left
    // the points are colinear
    } else {
        Orientation::Straight
    }
}

/// Computes the euclidean distance between two points.
pub fn euclid_dist(p0: &Point, p1: &Point) -> f32 {
    f32::sqrt(f32::powi(p0.0 - p1.0, 2) + f32::powi(p0.1 - p1.1, 2))
}

/// Checks if two line segments intersect.
/// 
/// This function returns `true` for the boundary case when an endpoint of one
/// line segment exists on the other line segment.
pub fn segments_intersect(l0: &LineSegment, l1: &LineSegment) -> bool {
    // compute the relative orientations for each straddle property

    // does segment l0 straddle the line l1?
    let d1 = cross_prod(l1.start(), l1.end(), l0.start());
    let d2 = cross_prod(l1.start(), l1.end(), l0.end());

    // does segment l1 straddle the segment l0?
    let d3 = cross_prod(l0.start(), l0.end(), l1.start());
    let d4 = cross_prod(l0.start(), l0.end(), l1.end());

    // opposite orientations must exist for both line segments to straddle each other's line
    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0)) && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0)) {
        true
    } else {
        // check boundary cases
        if d1 == 0.0 && on_segment(&l1, &l0.0) {
            true
        } else if d2 == 0.0 && on_segment(&l1, &l0.1) {
            true
        } else if d3 == 0.0 && on_segment(&l0, &l1.0) {
            true
        } else if d4 == 0.0 && on_segment(&l0, &l1.1) {
            true
        } else {
            false
        }
    }
}

/// Checks if the given point `p` is on the line segment `ls`.
pub fn on_segment(ls: &LineSegment, p: &Point) -> bool {
    ls.x_min() <= p.x() && p.x() <= ls.x_max() && ls.y_min() <= p.y() && p.y() <= ls.y_max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_cross_prod() {
        // example: colinear
        let result = cross_prod(&Point(0.0, 1.0), &Point(0.0, 4.0), &Point(0.0, 6.0));
        assert_eq!(result, 0.0);

        // example (a): CCW
        let result = cross_prod(&Point(0.0, 0.0), &Point(3.0, 6.0), &Point(-2.0, 4.0));
        assert_eq!(result, 24.0);

        // example (b): CW
        let result = cross_prod(&Point(0.0, 0.0), &Point(-2.0, 4.0), &Point(3.0, 6.0));
        assert_eq!(result, -24.0);
    }
    
    #[test]
    fn ut_direction() {
        // example: colinear
        let turn = direction(&Point(0.0, 1.0), &Point(0.0, 4.0), &Point(0.0, 6.0));
        assert_eq!(turn, Orientation::Straight);

        // example (a): CCW
        let turn = direction(&Point(0.0, 0.0), &Point(3.0, 6.0), &Point(-2.0, 4.0));
        assert_eq!(turn, Orientation::Left);

        // example (b): CW
        let turn = direction(&Point(0.0, 0.0), &Point(-2.0, 4.0), &Point(3.0, 6.0));
        assert_eq!(turn, Orientation::Right);
    }

    #[test]
    fn ut_on_segment() {
        // safely within the segment
        let result = on_segment(&LineSegment(Point(0.0, 4.0), Point(0.0, 10.0)), &Point(0.0, 6.0));
        assert_eq!(result, true);

        // on an endpoint
        let result = on_segment(&LineSegment(Point(0.0, 4.0), Point(2.0, 10.0)), &Point(2.0, 10.0));
        assert_eq!(result, true);

        let result = on_segment(&LineSegment(Point(0.0, 4.0), Point(2.0, 10.0)), &Point(0.0, 4.0));
        assert_eq!(result, true);

        // too far 'up'
        let result = on_segment(&LineSegment(Point(0.0, 4.0), Point(2.0, 10.0)), &Point(1.0, 11.0));
        assert_eq!(result, false);

        // too far 'down'
        let result = on_segment(&LineSegment(Point(0.0, 4.0), Point(2.0, 10.0)), &Point(1.0, 3.0));
        assert_eq!(result, false);

        // too far 'left'
        let result = on_segment(&LineSegment(Point(0.0, 4.0), Point(0.0, 10.0)), &Point(-1.0, 6.0));
        assert_eq!(result, false);

        // too far 'right'
        let result = on_segment(&LineSegment(Point(0.0, 4.0), Point(2.0, 10.0)), &Point(4.0, 7.0));
        assert_eq!(result, false);
    }

    #[test]
    fn ut_segments_intersect() {
        let line1 = LineSegment(Point(0.0, 0.0), Point(4.0, 4.0));
        let line2 = LineSegment(Point(0.0, 4.0), Point(4.0, 0.0));
        let result = segments_intersect(&line1, &line2);
        assert_eq!(result, true);

        let line1 = LineSegment(Point(0.0, 0.0), Point(4.0, 4.0));
        let line2 = LineSegment(Point(0.0, 4.0), Point(2.5, 1.5));
        let result = segments_intersect(&line1, &line2);
        assert_eq!(result, true);

        let line1 = LineSegment(Point(0.0, 0.0), Point(4.0, 4.0));
        let line2 = LineSegment(Point(0.0, 4.0), Point(2.0, 2.0));
        let result = segments_intersect(&line1, &line2);
        assert_eq!(result, true);

        let line1 = LineSegment(Point(0.0, 0.0), Point(4.0, 4.0));
        let line2 = LineSegment(Point(0.0, 4.0), Point(1.5, 2.5));
        let result = segments_intersect(&line1, &line2);
        assert_eq!(result, false);


        let line1 = LineSegment(Point(0.0, 0.0), Point(4.0, 4.0));
        let line2 = LineSegment(Point(0.0, 9.0), Point(9.0, 0.0));
        let result = segments_intersect(&line1, &line2);
        assert_eq!(result, false);
    }
}