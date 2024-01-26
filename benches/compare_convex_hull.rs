
use euclid::primitives::Point;

use criterion::{criterion_group, criterion_main, Criterion};

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

    group.bench_with_input("Brute Force", &input, |b, points| b.iter(|| convex_hull::brute_force(points)));
}

criterion_group!(benches, compare_convex_hulls);
criterion_main!(benches);