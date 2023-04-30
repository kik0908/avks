use std::sync::mpsc;
use std::{thread, time::Instant};

fn pi(n: i32, thread_count: usize) -> f64 {
    let per_thread = n as usize / thread_count;
    let ost = n as usize % thread_count;

    let obr_n = 1.0 / n as f64;

    let mut ans: f64 = 0.0;
    let mut start = 0;

    let (tx, rx) = mpsc::channel();
    for i in 0..thread_count - 1 {
        start = i * per_thread;
        let cur = start;
        let tx = tx.clone();
        thread::spawn(move || {
            let mut ans = 0.0;
            for i in cur..cur + per_thread {
                let tmp = (i as f64 + 0.5) * obr_n;
                ans += 4.0 / (1.0 + (tmp * tmp));
            }
            tx.send(ans).unwrap();
        });
        start = start + per_thread;
    }

    for i in start..start + per_thread + ost {
        let tmp = (i as f64 + 0.5) * obr_n;
        ans += 4.0 / (1.0 + (tmp * tmp));
    }

    for _ in 0..thread_count - 1 {
        ans += rx.recv().unwrap();
    }
    ans * obr_n
}

fn main() {
    let count = 1;
    let count_t = [1, 2, 4, 8, 10, 12];
    let Ns = [1_00, 1_000_000];

    println!("+--------+----------+----------------------+");
    println!("|   N    |  Threads |    Время  выполнения |");
    println!("|        |   count  |     в микросекундах  |");
    println!("+--------+----------+----------------------+");

    for t_count in count_t {
        for n in Ns {
            let mut all = 0;
            for _ in 0..count {
                let start = Instant::now();
                pi(n, t_count);
                all += start.elapsed().as_micros()
            }
            println!("|{n:>8}|{t_count:>10}|{a:>19}мкс|", a = all/count);
            println!("+--------+----------+----------------------+");
        }
    }
}
