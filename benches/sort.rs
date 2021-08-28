use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sort_for_short_array::sorting_network::{sort4, sort5};

fn std_sort4_i32(c: &mut Criterion) {
    c.bench_function("std_sort4_i32", |b| {
        b.iter(|| {
            let mut arr: [i32; 4] = black_box([9, 1, 8, 1]);
            arr.sort();
        });
    });
}

fn sorting_network_sort4_i32(c: &mut Criterion) {
    c.bench_function("sorting_network_sort4_i32", |b| {
        b.iter(|| {
            let mut arr: [i32; 4] = black_box([9, 1, 8, 1]);
            sort4(&mut arr);
        });
    });
}

fn std_sort5_i32(c: &mut Criterion) {
    c.bench_function("std_sort5_i32", |b| {
        b.iter(|| {
            let mut arr: [i32; 5] = black_box([9, 1, 8, 1, 3]);
            arr.sort();
        });
    });
}

fn sorting_network_sort5_i32(c: &mut Criterion) {
    c.bench_function("sorting_network_sort5_i32", |b| {
        b.iter(|| {
            let mut arr: [i32; 5] = black_box([9, 1, 8, 1, 3]);
            sort5(&mut arr);
        });
    });
}

criterion_group!(
    benches,
    std_sort4_i32,
    sorting_network_sort4_i32,
    std_sort5_i32,
    sorting_network_sort5_i32,
);
criterion_main!(benches);
