use std::{
    sync::{
        atomic::{AtomicU32, Ordering::Relaxed},
        mpsc, Arc,
    },
    thread::{self, sleep},
    time::{Duration, Instant},
};

const THREADS_COUNT: usize = 10;

fn single(arr: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for i in arr {
        sum += i;
    }
    sum
}

fn atomic(arr: Arc<Vec<u32>>, threads_count: usize) -> u32 {
    let sum = Arc::new(AtomicU32::new(0));

    let length_child_arr = (arr.len() as f32 / threads_count as f32).ceil() as usize;
    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(threads_count);

    for i in 1..threads_count {
        let cur_sum = Arc::clone(&sum);
        let cur_arr = Arc::clone(&arr);
        let start = i * length_child_arr;
        let finish = start + length_child_arr;

        handlers.push(thread::spawn(move || {
            for j in start..(finish).min(cur_arr.len()) {
                cur_sum.fetch_add(cur_arr[j], Relaxed);
            }
        }));
    }

    for j in 0..length_child_arr {
        sum.fetch_add(arr[j], Relaxed);
    }

    for thread in handlers {
        thread.join().unwrap();
    }

    sum.load(Relaxed)
}

fn reduction(arr: Arc<Vec<u32>>, threads_count: usize) -> u32 {
    let mut sum = 0;

    let length_child_arr = (arr.len() as f32 / threads_count as f32).ceil() as usize;

    let (tx, rx) = mpsc::channel();

    for i in 1..threads_count {
        let cur_arr = Arc::clone(&arr);
        let start = i * length_child_arr;
        let finish = start + length_child_arr;

        let cur_tx = tx.clone();

        thread::spawn(move || {
            let mut sum = 0;
            for j in &cur_arr[start..(finish).min(cur_arr.len())] {
                sum += *j; //cur_arr[j];
            }
            cur_tx.send(sum).unwrap();
        });
    }

    for j in 0..length_child_arr {
        sum += arr[j];
    }

    for _ in 1..threads_count {
        sum += rx.recv().unwrap();
    }

    sum
}

fn main() {
    println!("Start");
    let mut root_arr = Vec::new();
    let arr_size: usize = 1_000_000_000;

    for _ in 0..arr_size {
        root_arr.push(1);
    }

    let start = Instant::now();
    let ans1 = single(&root_arr);
    println!(
        "Single version {}secs, answer {ans1}",
        start.elapsed().as_secs_f32()
    );

    let arr2 = Arc::new(root_arr);
    let arr3 = Arc::clone(&arr2);

    println!("Start pause");
    sleep(Duration::new(5, 0));

    let start = Instant::now();
    let ans2 = atomic(arr2, THREADS_COUNT);
    println!(
        "Atomic version {}secs, answer {ans2}",
        start.elapsed().as_secs_f32()
    );

    let start = Instant::now();
    let ans3 = reduction(arr3, THREADS_COUNT);
    println!(
        "Reduct version {}secs, answer {ans3}",
        start.elapsed().as_secs_f32()
    );
}
