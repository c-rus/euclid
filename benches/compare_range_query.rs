use euclid::primitives::{Point, Region};

use criterion::{criterion_group, criterion_main, Criterion};

fn compare_range_query(c: &mut Criterion) {
    use euclid::kd_tree::KdTree;
    use euclid::range_tree::RangeTree;

    let mut group = c.benchmark_group("Range Query");

    let sets = vec![
        (100, "data/points100uni.txt"),
        (1_000, "data/points1000uni.txt"),
        (10_000, "data/points10000uni.txt"),
        (65_536, "data/points65536uni.txt"),
        (100_000, "data/points100000uni.txt"),
    ];

    // the region for the uniform distribution
    let region = Region::new(Point::from((1.0, 4.0)), Point::from((9.0, 19.0)));
    // let region = Region::new(Point::from((0.0, 0.0)), Point::from((100.0, 100.0)));

    for (i, filepath) in &sets {
        let points = euclid::read_points(filepath);

        let tree = KdTree::construct(points.clone());
        group.bench_function(format!("kd-tree {}", i), |b| {
            b.iter(|| tree.range_query(&region))
        });

        let tree = RangeTree::construct(points);
        group.bench_function(format!("range-tree {}", i), |b| {
            b.iter(|| tree.range_query(&region))
        });
    }
}

criterion_group!(benches, compare_range_query);
criterion_main!(benches);
