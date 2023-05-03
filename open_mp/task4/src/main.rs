use rand::Fill;
use std::{
    ptr::swap,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Instant,
    vec,
};

const THREAD_COUNT: usize = 6;
const ARR_SIZE: usize = 1_000_000_00;

#[derive(Debug)]
struct MexicoChecker {
    now: usize,
    len: usize,
}

impl MexicoChecker {
    fn new(len: usize) -> Self {
        MexicoChecker { now: 0, len: len }
    }

    fn add(&mut self, num: usize) {
        self.now += num;
    }

    fn check(&self) -> bool {
        self.len == self.now
    }
}

fn insertion_sort(arr: &mut [u32]) {
    for i in 1..arr.len() {
        let mut j = i as i32 - 1;
        while j >= 0 && arr[j as usize] > arr[j as usize + 1] {
            unsafe { swap(&mut arr[j as usize], &mut arr[j as usize + 1]) }
            j -= 1;
        }
    }
}

fn partition(arr: &mut [u32]) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let pivot = arr[(left + right) / 2];
    while left <= right {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }
        if left >= right {
            break;
        }
        unsafe {
            swap(&mut arr[left], &mut arr[right]);
            left += 1;
            right -= 1
        }
    }
    right
}

fn quicksort_single(arr: &mut [u32]) {
    if arr.len() <= 25 {
        insertion_sort(arr);
        return;
    }
    let q = partition(arr);
    quicksort_single(&mut arr[..q + 1]);
    quicksort_single(&mut arr[q + 1..]);
}

fn quicksort_multi(arr: &mut Vec<u32>) {
    let mut threads = Vec::with_capacity(THREAD_COUNT);
    let mut channels = Vec::with_capacity(THREAD_COUNT);

    let (tx, rx) = mpsc::channel::<Option<&mut [u32]>>();
    let checker = Arc::new(Mutex::new(MexicoChecker::new(arr.len())));

    for _ in 0..THREAD_COUNT {
        let cur_tx = tx.clone();
        let (tx, cur_rx) = mpsc::channel::<&mut [u32]>();
        channels.push(tx);
        let checker = Arc::clone(&checker);
        threads.push(thread::spawn(move || {
            for arr in cur_rx {
                if arr.len() <= 25 {
                    insertion_sort(arr);
                    let tmp: bool;
                    {
                        let mut a = checker.lock().unwrap();
                        a.add(arr.len());
                        tmp = a.check();
                    };
                    if tmp {
                        cur_tx.send(None).unwrap();
                    }
                    continue;
                }
                let q = partition(arr);
                let (left, right) = arr.split_at_mut(q + 1);

                cur_tx.send(Some(left)).unwrap();
                cur_tx.send(Some(right)).unwrap();
            }
        }));
    }

    let mut channels_iter = channels.iter().cycle();
    unsafe {
        let s = &mut arr[..];
        let len = s.len();
        let ptr = s.as_mut_ptr();
        let slice = std::slice::from_raw_parts_mut(ptr, len);
        channels_iter.next().unwrap().send(slice).unwrap();
    }

    for i in &rx {
        match i {
            Some(i) => channels_iter.next().unwrap().send(i).unwrap(),
            None => break,
        }
    }

    for i in channels {
        drop(i)
    }

    for i in threads {
        i.join().unwrap();
    }
}

fn check(f: &Vec<u32>, s: &Vec<u32>, text: &str) {
    for i in 0..f.len() {
        if f[i] != s[i] {
            panic!("{}", text)
        }
    }
}

fn main() {
    let mut first: Vec<u32> = vec![0; ARR_SIZE];
    first.try_fill(&mut rand::thread_rng()).unwrap();
    let mut second = first[..].to_vec();

    let start = Instant::now();
    quicksort_single(&mut first[..]);
    let delta = start.elapsed().as_millis();
    println!("single version - {}ms", delta);

    let start = Instant::now();
    quicksort_multi(&mut second);
    let delta = start.elapsed().as_millis();
    println!("Multithread version - {}ms", delta);
    check(&first, &second, "multi wrong!");
}
