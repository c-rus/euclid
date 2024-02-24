use euclid::primitives::Region;
use euclid::{self, kd_tree::KdTree, range_tree::RangeTree};
use std::env;

fn main() {
    let mut args = env::args().collect::<Vec<String>>().into_iter();

    // skip executable name
    args.next().unwrap();
    // algorithm name
    let algo = args.next().unwrap();

    match algo.as_str() {
        "upperlower" => {
            // read inputs
            let points = euclid::read_points(&args.next().unwrap());
            // compute convex hull
            let hull = euclid::convex_hull::upper_lower(&points);
            // write results
            euclid::write_points(&args.next().unwrap(), hull);
        }
        "brutehull" => {
            // read inputs
            let points = euclid::read_points(&args.next().unwrap());
            // compute convex hull
            let hull = euclid::convex_hull::brute_force(&points);
            // write results
            euclid::write_points(&args.next().unwrap(), hull);
        }
        "stairs" => {
            // read inputs
            let points = euclid::read_points(&args.next().unwrap());
            // compute staircase
            let stairs = euclid::staircase::staircase(&points);
            // write results
            euclid::write_points(&args.next().unwrap(), stairs);
        }
        "kd" => {
            // read point set
            let points = euclid::read_points::<f32>(&args.next().unwrap());
            // read range
            let region = {
                let range = euclid::read_points::<f32>(&args.next().unwrap());
                Region::new(*range.get(0).unwrap(), *range.get(1).unwrap())
            };
            // construct kd-tree
            let kd_tree = KdTree::construct(points);
            // perform range query on kd-tree
            let result = kd_tree.range_query(&region);
            // write results
            euclid::write_points::<f32>(&args.next().unwrap(), result);
        }
        "range" => {
            // read point set
            let points = euclid::read_points::<f32>(&args.next().unwrap());
            // read range
            let region = {
                let range = euclid::read_points::<f32>(&args.next().unwrap());
                Region::new(*range.get(0).unwrap(), *range.get(1).unwrap())
            };
            // construct kd-tree
            let range_tree = RangeTree::construct(points);
            // perform range query on kd-tree
            let result = range_tree.range_query(&region);
            // write results
            euclid::write_points::<f32>(&args.next().unwrap(), result);
        }
        _ => {
            panic!()
        }
    }
}
