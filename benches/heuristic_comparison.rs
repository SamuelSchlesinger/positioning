use std::collections::BTreeSet;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use positioning::{
    pathfinding::{all_pairs_shortest_paths, HammingDistance, Heuristic},
    Position,
};

fn benchmark(c: &mut Criterion) {
    let mut statically_open = BTreeSet::new();
    for n in vec![60].into_iter() {
        for x in 0..n {
            for y in 0..n {
                if (y == 2 && x > 2 && x < n - 2) || (x == 2 && y > 2) {
                    continue;
                }
                statically_open.insert(Position::new(x, y, 0));
            }
        }
        let all_pairs = all_pairs_shortest_paths(&statically_open);

        let mut dynamically_open = statically_open.clone();
        dynamically_open.remove(&Position::new(n - 1, n / 2, 0));

        let hamming_distance = HammingDistance;

        c.bench_with_input(
            BenchmarkId::new("all_pairs", format!("n = {}", n)),
            &dynamically_open,
            |b, d| {
                b.iter(|| {
                    let _ = all_pairs.find_shortest_path(
                        &d,
                        Position::new(n - 1, n - 1, 0),
                        Position::new(0, n - 1, 0),
                    );
                });
            },
        );
        c.bench_with_input(
            BenchmarkId::new("hamming", format!("n = {}", n)),
            &dynamically_open,
            |b, d| {
                b.iter(|| {
                    let _ = hamming_distance.find_shortest_path(
                        &d,
                        Position::new(n - 1, n - 1, 0),
                        Position::new(0, n - 1, 0),
                    );
                });
            },
        );
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
