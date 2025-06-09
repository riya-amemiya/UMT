use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use std::hint::black_box;
use umt_rust::array::umt_ultra_number_sort;

fn generate_random_array(size: usize, range: std::ops::Range<f64>) -> Vec<f64> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random_range(range.clone())).collect()
}

fn generate_integer_array(size: usize, range: std::ops::Range<i32>) -> Vec<f64> {
    let mut rng = rand::rng();
    (0..size)
        .map(|_| rng.random_range(range.clone()) as f64)
        .collect()
}

fn bench_small_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_arrays");

    let small_array = generate_random_array(10, -100.0..100.0);
    group.bench_function("10_elements_ascending", |b| {
        b.iter(|| {
            let mut arr = small_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.bench_function("10_elements_descending", |b| {
        b.iter(|| {
            let mut arr = small_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), false);
        })
    });

    group.finish();
}

fn bench_medium_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("medium_arrays");

    let medium_array = generate_random_array(100, -1000.0..1000.0);
    group.bench_function("100_elements_ascending", |b| {
        b.iter(|| {
            let mut arr = medium_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.bench_function("100_elements_descending", |b| {
        b.iter(|| {
            let mut arr = medium_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), false);
        })
    });

    group.finish();
}

fn bench_large_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_arrays");

    let large_array = generate_random_array(1000, -10000.0..10000.0);
    group.bench_function("1000_elements_ascending", |b| {
        b.iter(|| {
            let mut arr = large_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.bench_function("1000_elements_descending", |b| {
        b.iter(|| {
            let mut arr = large_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), false);
        })
    });

    let very_large_array = generate_random_array(10000, -100000.0..100000.0);
    group.bench_function("10000_elements_ascending", |b| {
        b.iter(|| {
            let mut arr = very_large_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.bench_function("100000_elements_ascending", |b| {
        b.iter(|| {
            let mut arr = generate_random_array(100000, -1000000.0..1000000.0);
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.bench_function("1000000_elements_ascending", |b| {
        b.iter(|| {
            let mut arr = generate_random_array(1000000, -10000000.0..10000000.0);
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.bench_function("10000000_elements_ascending", |b| {
        b.iter(|| {
            let mut arr = generate_random_array(10000000, -100000000.0..100000000.0);
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.finish();
}

fn bench_special_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("special_cases");

    let mut sorted_array: Vec<f64> = (0..1000).map(|i| i as f64).collect();
    group.bench_function("already_sorted_ascending", |b| {
        b.iter(|| {
            let mut arr = sorted_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    sorted_array.reverse();
    let reverse_sorted = sorted_array.clone();
    group.bench_function("reverse_sorted_to_ascending", |b| {
        b.iter(|| {
            let mut arr = reverse_sorted.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let same_elements = vec![42.0; 1000];
    group.bench_function("all_same_elements", |b| {
        b.iter(|| {
            let mut arr = same_elements.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let mut rng = rand::rng();
    let duplicates: Vec<f64> = (0..1000)
        .map(|_| (rng.random_range(0..10) as f64))
        .collect();
    group.bench_function("many_duplicates", |b| {
        b.iter(|| {
            let mut arr = duplicates.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.finish();
}

fn bench_integer_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("integer_arrays");

    let small_range_integers = generate_integer_array(100, 0..50);
    group.bench_function("small_range_integers", |b| {
        b.iter(|| {
            let mut arr = small_range_integers.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let large_integers = generate_integer_array(200, -1000..1000);
    group.bench_function("large_range_integers", |b| {
        b.iter(|| {
            let mut arr = large_integers.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let positive_integers = generate_integer_array(150, 1..1000);
    group.bench_function("positive_integers_only", |b| {
        b.iter(|| {
            let mut arr = positive_integers.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let negative_integers = generate_integer_array(150, -1000..-1);
    group.bench_function("negative_integers_only", |b| {
        b.iter(|| {
            let mut arr = negative_integers.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.finish();
}

fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    let empty: Vec<f64> = vec![];
    group.bench_function("empty_array", |b| {
        b.iter(|| {
            let mut arr = empty.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let single = vec![42.0];
    group.bench_function("single_element", |b| {
        b.iter(|| {
            let mut arr = single.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let mut with_nan = generate_random_array(100, -100.0..100.0);
    with_nan[10] = f64::NAN;
    with_nan[50] = f64::NAN;
    with_nan[90] = f64::NAN;
    group.bench_function("array_with_nan", |b| {
        b.iter(|| {
            let mut arr = with_nan.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    let mut with_infinity = generate_random_array(100, -100.0..100.0);
    with_infinity[20] = f64::INFINITY;
    with_infinity[60] = f64::NEG_INFINITY;
    group.bench_function("array_with_infinity", |b| {
        b.iter(|| {
            let mut arr = with_infinity.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.finish();
}

fn bench_comparison_with_std(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison_with_std");

    let test_array = generate_random_array(1000, -1000.0..1000.0);

    group.bench_function("umt_ultra_number_sort", |b| {
        b.iter(|| {
            let mut arr = test_array.clone();
            umt_ultra_number_sort(black_box(&mut arr), true);
        })
    });

    group.bench_function("std_sort_by", |b| {
        b.iter(|| {
            let mut arr = test_array.clone();
            arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        })
    });

    group.bench_function("std_sort_unstable_by", |b| {
        b.iter(|| {
            let mut arr = test_array.clone();
            arr.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_small_arrays,
    bench_medium_arrays,
    bench_large_arrays,
    bench_special_cases,
    bench_integer_arrays,
    bench_edge_cases,
    bench_comparison_with_std
);

criterion_main!(benches);
