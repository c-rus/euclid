use crate::primitives::{LineSegment, Region, Point};


#[derive(Debug, PartialEq)]
struct Trapezoid {
    top: LineSegment,
    bottom: LineSegment,
    leftp: Point<f32>,
    rightp: Point<f32>,
}

impl Trapezoid {
    // Determines if a line segment `s` intersects the given trapezoid.
    pub fn intersects(s: &LineSegment) -> bool {
        todo!();
    }
}



/// Computes the trapezoidal map T and search structure D for a set of
/// non-crossing line segments `segments`.
pub fn trapezoidal_map(segments: &Vec<LineSegment>) -> () {
    // determine bounding box R
    let bounding_box = Region::<f32>::new(Point::<f32>::from((0.0, 0.0)), Point::<f32>::from((18.0, 12.0)));
    
    // initialize trapezoidal map and search structure
    let mut t_map: Vec<Trapezoid> = vec![];
    let mut s_struct = todo!();
    
    // compute random permutation of line segments
    // ...skip    
    for (i, seg) in segments.iter().enumerate() {
        println!("{}: {}", i, seg);
    }

    // incrementally add each line segment to the known solution
    for (i, seg) in segments.iter().enumerate() {
        
    }

    todo!()
}