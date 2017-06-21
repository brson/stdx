extern crate threadpool;
extern crate num_cpus;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    // Get the number of cpus on current machine
    let n_workers = num_cpus::get();
    let n_jobs = 8;

    // Create the thread pool with amount of workers equal to cores
    let pool = ThreadPool::new(n_workers);

    // Create transmitter and receiver channel
    let (tx, rx) = channel();

    // For each job grab a free worker from the pool and execute
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1).unwrap();
        });
    }

    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}
