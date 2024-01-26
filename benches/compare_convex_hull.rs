
use euclid::primitives::Point;

use criterion::{criterion_group, criterion_main, Criterion};
use rand;

/// Generate randomly sampled points as input to benchmarks.
fn sample_points(size: usize) -> Vec<Point> {
    let mut points = Vec::new();
    for _i in 0..size {
        points.push(Point::from((rand::random::<f32>() * 512.0, rand::random::<f32>() * 512.0)));
    }
    points
}

fn compare_convex_hulls(c: &mut Criterion) {
    use euclid::convex_hull;

    let mut group = c.benchmark_group("Convex Hull");

    let input = vec![
        Point::from((0.0, 0.0)),
        Point::from((2.0, 10.0)),
        Point::from((5.0, 1.0)),
        Point::from((2.0, 4.0)),
        Point::from((3.0, 2.0)),
    ];

    group.bench_with_input("Brute Force (5)", &input, |b, points| b.iter(|| convex_hull::brute_force(points)));
    group.bench_with_input("Upper Lower (5)", &input, |b, points| b.iter(|| convex_hull::upper_lower(points)));

    let input = sample_points(1_000);

    group.bench_with_input("Brute Force (1000)", &input, |b, points| b.iter(|| convex_hull::brute_force(points)));
    group.bench_with_input("Upper Lower (1000)", &input, |b, points| b.iter(|| convex_hull::upper_lower(points)));
}

criterion_group!(benches, compare_convex_hulls);
criterion_main!(benches);