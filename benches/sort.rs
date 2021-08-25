use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sort_for_short_array::sorting_network::sort5;

fn std_sort5(c: &mut Criterion) {
    c.bench_function("std_sort5", |b| {
        b.iter(|| {
            let mut arr = black_box([9, 1, 8, 1, 3]);
            arr.sort();
        });
    });
}

fn sorting_network_sort5(c: &mut Criterion) {
    c.bench_function("sorting_network_sort5", |b| {
        b.iter(|| {
            let mut arr = black_box([9, 1, 8, 1, 3]);
            sort5(&mut arr);
        });
    });
}

criterion_group!(benches, std_sort5, sorting_network_sort5);
criterion_main!(benches);
