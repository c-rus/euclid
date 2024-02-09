
use std::env;
use euclid;

fn main() {
    let mut args = env::args().collect::<Vec<String>>().into_iter();

    // skip executable name
    args.next().unwrap();
    // algorithm name
    let algo = args.next().unwrap();
    let input_file = args.next().unwrap();
    let output_file = args.next().unwrap();

    match algo.as_str() {
        "upperlower" => {
            // read inputs
            let points = euclid::read_points(&input_file);
            // compute convex hull
            let hull = euclid::convex_hull::upper_lower(&points);
            // write results
            euclid::write_points(&output_file, hull);
        }
        "brutehull" => {
            // read inputs
            let points = euclid::read_points(&input_file);
            // compute convex hull
            let hull = euclid::convex_hull::brute_force(&points);
            // write results
            euclid::write_points(&output_file, hull);
        }
        "stairs" => {
            // read inputs
            let points = euclid::read_points(&input_file);
            // compute staircase
            let stairs = euclid::staircase::staircase(&points);
            // write results
            euclid::write_points(&output_file, stairs);
        }
        _ => {
            panic!()
        }
    }
}