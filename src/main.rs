use euclid;

fn main() {
    // read inputs
    let points = euclid::read_points("data/points.txt");
    
    // compute convex hull
    let hull = euclid::convex_hull::brute_force(&points);

    // write results
    euclid::write_points("data/hull.txt", hull);
}