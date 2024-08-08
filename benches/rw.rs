use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use criterion::{black_box, Criterion, criterion_group, criterion_main};

const SIZE: usize = 1_000;

fn test_array_read(c: &mut Criterion) {
    let mut vec: Vec<usize> = (0..SIZE).map(|i| black_box(i)).collect();

    let mut sum = 0;
    c.bench_function("test_array_read", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                sum += black_box(vec[i]);
            }
        });
    });
    // c.bench_function("test_array_read_no_blackbox", |b| {
    //     b.iter(|| {
    //         for i in 0..SIZE {
    //             sum += vec[i];
    //         }
    //     });
    // });
    println!("Sum is {sum}!");
}

fn test_array_read_atomic(c: &mut Criterion) {
    let mut vec: Vec<AtomicUsize> = (0..SIZE).map(|i| black_box(AtomicUsize::new(i))).collect();

    let mut sum = 0;
    c.bench_function("test_array_read_atomic", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                sum += black_box(vec[i].load(Ordering::Relaxed));
            }
        });
    });
    // c.bench_function("test_array_read_atomic_no_blackbox", |b| {
    //     b.iter(|| {
    //         for i in 0..SIZE {
    //             sum += vec[i].load(Ordering::Relaxed);
    //         }
    //     });
    // });
    println!("Sum is {sum}!");
}

fn test_array_read_refcell(c: &mut Criterion) {
    let mut vec: Vec<RefCell<usize>> = (0..SIZE).map(|i| black_box(RefCell::new(i))).collect();

    let mut sum = 0;
    c.bench_function("test_array_read_refcell", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                sum += black_box(*vec[i].borrow());
            }
        });
    });
    // c.bench_function("test_array_read_refcell_no_blackbox", |b| {
    //     b.iter(|| {
    //         for i in 0..SIZE {
    //             sum += *vec[i].borrow();
    //         }
    //     });
    // });
    println!("Sum is {sum}!");
}


fn test_array_write(c: &mut Criterion) {
    let mut vec: Vec<usize> = (0..SIZE).map(|i| black_box(i)).collect();

    c.bench_function("test_array_write", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                vec[i] = black_box(15);
            }
        });
    });
    // c.bench_function("test_array_write_no_blackbox", |b| {
    //     b.iter(|| {
    //         for i in 0..SIZE {
    //             vec[i] = 15;
    //         }
    //     });
    // });
}

fn test_array_write_atomic(c: &mut Criterion) {
    let mut vec: Vec<AtomicUsize> = (0..SIZE).map(|i| black_box(AtomicUsize::new(i))).collect();

    c.bench_function("test_array_write_atomic", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                vec[i].store(black_box(15), Ordering::Relaxed);
            }
        });
    });
    // c.bench_function("test_array_write_atomic_no_blackbox", |b| {
    //     b.iter(|| {
    //         for i in 0..SIZE {
    //             vec[i].store(15, Ordering::Relaxed);
    //         }
    //     });
    // });
}

fn test_array_write_refcell(c: &mut Criterion) {
    let mut vec: Vec<RefCell<usize>> = (0..SIZE).map(|i| black_box(RefCell::new(i))).collect();

    c.bench_function("test_array_write_refcell", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                *vec[i].borrow_mut() = black_box(15);
            }
        });
    });
    // c.bench_function("test_array_write_refcell_no_blackbox", |b| {
    //     b.iter(|| {
    //         for i in 0..SIZE {
    //             *vec[i].borrow_mut() = 15;
    //         }
    //     });
    // });
}

criterion_group!(benches, test_array_read, test_array_read_atomic, test_array_read_refcell, test_array_write, test_array_write_atomic, test_array_write_refcell);
criterion_main!(benches);