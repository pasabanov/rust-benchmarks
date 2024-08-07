#![feature(test)]

extern crate test;

use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use test::{Bencher, black_box};

const SIZE:usize = 1_000_000;

#[bench]
fn test_array_read_refcell(b: &mut Bencher) {
    let mut vec: Vec<RefCell<usize>> = (0..SIZE).map(|i| black_box(RefCell::new(i))).collect();

    let mut sum = 0;
    b.iter(|| {
        for i in 0..SIZE {
            sum += black_box(*vec[i].borrow());
        }
    });
    println!("Sum is {sum}!");
}

#[bench]
fn test_array_read_atomic(b: &mut Bencher) {
    let mut vec: Vec<AtomicUsize> = (0..SIZE).map(|i| black_box(AtomicUsize::new(i))).collect();

    let mut sum = 0;
    b.iter(|| {
        for i in 0..SIZE {
            sum += black_box(vec[i].load(Ordering::Relaxed));
        }
    });
    println!("Sum is {sum}!");
}

#[bench]
fn test_array_read(b: &mut Bencher) {
    let mut vec: Vec<usize> = (0..SIZE).map(|i| black_box(i)).collect();

    let mut sum = 0;
    b.iter(|| {
        for i in 0..SIZE {
            sum += black_box(vec[i]);
        }
    });
    println!("Sum is {sum}!");
}


#[bench]
fn test_array_write_refcell(b: &mut Bencher) {
    let mut vec: Vec<RefCell<usize>> = (0..SIZE).map(|i| black_box(RefCell::new(i))).collect();

    b.iter(|| {
        for i in 0..SIZE {
            *vec[i].borrow_mut() = black_box(15);
        }
    });
}

#[bench]
fn test_array_write_atomic(b: &mut Bencher) {
    let mut vec: Vec<AtomicUsize> = (0..SIZE).map(|i| black_box(AtomicUsize::new(i))).collect();

    b.iter(|| {
        for i in 0..SIZE {
            vec[i].store(black_box(15), Ordering::Relaxed);
        }
    });
}

#[bench]
fn test_array_write(b: &mut Bencher) {
    let mut vec: Vec<usize> = (0..SIZE).map(|i| black_box(i)).collect();

    b.iter(|| {
        for i in 0..SIZE {
            vec[i] = black_box(15);
        }
    });
}




#[bench]
fn test_array_read_refcell_no_blackbox(b: &mut Bencher) {
    let mut vec: Vec<RefCell<usize>> = (0..SIZE).map(|i| RefCell::new(i)).collect();

    let mut sum = 0;
    b.iter(|| {
        for i in 0..SIZE {
            sum += *vec[i].borrow();
        }
    });
    println!("Sum is {sum}!");
}

#[bench]
fn test_array_read_atomic_no_blackbox(b: &mut Bencher) {
    let mut vec: Vec<AtomicUsize> = (0..SIZE).map(|i| AtomicUsize::new(i)).collect();

    let mut sum = 0;
    b.iter(|| {
        for i in 0..SIZE {
            sum += vec[i].load(Ordering::Relaxed);
        }
    });
    println!("Sum is {sum}!");
}

#[bench]
fn test_array_read_no_blackbox(b: &mut Bencher) {
    let mut vec: Vec<usize> = (0..SIZE).map(|i| i).collect();

    let mut sum = 0;
    b.iter(|| {
        for i in 0..SIZE {
            sum += vec[i];
        }
    });
    println!("Sum is {sum}!");
}


#[bench]
fn test_array_write_refcell_no_blackbox(b: &mut Bencher) {
    let mut vec: Vec<RefCell<usize>> = (0..SIZE).map(|i| RefCell::new(i)).collect();

    b.iter(|| {
        for i in 0..SIZE {
            *vec[i].borrow_mut() = 15;
        }
    });
}

#[bench]
fn test_array_write_atomic_no_blackbox(b: &mut Bencher) {
    let mut vec: Vec<AtomicUsize> = (0..SIZE).map(|i| AtomicUsize::new(i)).collect();

    b.iter(|| {
        for i in 0..SIZE {
            vec[i].store(15, Ordering::Relaxed);
        }
    });
}

#[bench]
fn test_array_write_no_blackbox(b: &mut Bencher) {
    let mut vec: Vec<usize> = (0..SIZE).map(|i| i).collect();

    b.iter(|| {
        for i in 0..SIZE {
            vec[i] = 15;
        }
    });
}
