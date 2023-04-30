const MATRIX_SIZE: usize = 1000;
const THREAD_COUNT: usize = 10;
use std::{
    sync::{atomic::AtomicI32, Arc},
    thread,
    time::Instant,
};

use rand::Rng;

fn one_thread(mat1: &Vec<Vec<i32>>, mat2: &Vec<Vec<i32>>, rez: &mut Vec<Vec<i32>>) {
    let mut mat3 = Vec::with_capacity(MATRIX_SIZE);
    for i in 0..MATRIX_SIZE {
        mat3.push(Vec::with_capacity(MATRIX_SIZE));
        for j in 0..MATRIX_SIZE {
            mat3[i].push(mat2[j][i])
        }
    }
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            for k in 0..MATRIX_SIZE {
                rez[i][j] += mat1[i][k] * mat3[j][k];
            }
        }
    }
}

fn multi_thread(mat1: Arc<Vec<Vec<i32>>>, mat2: Arc<Vec<Vec<i32>>>, rez: Arc<Vec<Vec<AtomicI32>>>) {
    let per_thread = MATRIX_SIZE / THREAD_COUNT;
    let ost = MATRIX_SIZE % THREAD_COUNT;

    let mut threads = Vec::with_capacity(THREAD_COUNT);

    let mut mat3 = Vec::with_capacity(MATRIX_SIZE);
    for i in 0..MATRIX_SIZE {
        mat3.push(Vec::with_capacity(MATRIX_SIZE));
        for j in 0..MATRIX_SIZE {
            mat3[i].push(mat2[j][i])
        }
    }
    let mat3 = Arc::new(mat3);

    let mut start = 0;
    for i in 0..THREAD_COUNT - 1 {
        let mat1 = Arc::clone(&mat1);
        let mat2 = Arc::clone(&mat3);
        start = i * per_thread;
        let rez = Arc::clone(&rez);
        threads.push(thread::spawn(move || {
            for i in start..start + per_thread {
                for j in 0..MATRIX_SIZE {
                    for k in 0..MATRIX_SIZE {
                        rez[i][j].fetch_add(
                            mat1[i][k] * mat2[j][k],
                            std::sync::atomic::Ordering::Relaxed,
                        );
                    }
                }
            }
        }));
        start += per_thread;
    }

    for i in start..start + per_thread + ost {
        for j in 0..MATRIX_SIZE {
            for k in 0..MATRIX_SIZE {
                rez[i][j].fetch_add(
                    mat1[i][k] * mat3[j][k],
                    std::sync::atomic::Ordering::Relaxed,
                );
            }
        }
    }

    for i in threads {
        i.join().unwrap();
    }
}

fn check(first: Vec<Vec<i32>>, second: Arc<Vec<Vec<AtomicI32>>>) {
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            if first[i][j] != second[i][j].load(std::sync::atomic::Ordering::Relaxed) {
                panic!("error, wrong multipl.")
            }
        }
    }
}

fn main() {
    let mut matrix1 = Vec::with_capacity(MATRIX_SIZE);
    let mut matrix2 = Vec::with_capacity(MATRIX_SIZE);
    for i in 0..MATRIX_SIZE {
        matrix1.push(Vec::with_capacity(MATRIX_SIZE));
        matrix2.push(Vec::with_capacity(MATRIX_SIZE));

        for _ in 0..MATRIX_SIZE {
            matrix1[i].push(rand::thread_rng().gen_range(0..10));
            matrix2[i].push(rand::thread_rng().gen_range(0..10));
        }
    }

    let mut rez1 = Vec::with_capacity(MATRIX_SIZE);
    let mut rez2 = Vec::with_capacity(MATRIX_SIZE);
    for i in 0..MATRIX_SIZE {
        rez1.push(Vec::with_capacity(MATRIX_SIZE));
        rez2.push(Vec::with_capacity(MATRIX_SIZE));
        for _ in 0..MATRIX_SIZE {
            rez1[i].push(0);
            rez2[i].push(AtomicI32::new(0));
        }
    }
    let start = Instant::now();
    one_thread(&matrix1, &matrix2, &mut rez1);
    let delta = start.elapsed().as_millis();
    println!("Single version - {}ms", delta);

    let rez2 = Arc::new(rez2);
    let start = Instant::now();
    multi_thread(Arc::new(matrix1), Arc::new(matrix2), Arc::clone(&rez2));
    let delta = start.elapsed().as_millis();
    println!("{} threads version - {}ms", THREAD_COUNT, delta);
    check(rez1, rez2);
}
