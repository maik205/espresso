use std::{ sync::{ mpsc, Mutex, MutexGuard }, thread };

use super::Job;

struct LockingJob<'a> {
    job: Job,
    thread_guard: MutexGuard<'a, Worker>,
}
/// ## Info
/// The thread pool holds a number of threads to process concurrently
/// This implementation is a first-available-first-service thread pool, where the threads are checked for availability linearly
/// ## Todo:
/// - Finish the executing method
/// - Implement a available thread vector for pigeon-hole thread scheduling (tracking threads availability instead of blind search)
/// ## Panics
/// If the number of threads is <= 0
pub struct ThreadPool {
    workers: Vec<Mutex<Worker>>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
    work_chann: Option<mpsc::Sender<Job>>,
}
/// A worker-mutex based thread pool implementation
/// ## Pros:
/// - Health-checkable (% of workers busy) for scaling and sharding if necessary
/// ## Cons:
/// Checks incur too much runtime cost due to high Mutex usage.
impl ThreadPool {
    pub fn new(threads_num: usize) -> ThreadPool {
        assert!(threads_num > 0);

        let mut workers: Vec<Mutex<Worker>> = vec![];
        for i in 0..threads_num {
            workers.push(Mutex::new(Worker::new(i)));
        }
        println!("Created {threads_num} workers");
        ThreadPool { workers }
    }

    pub fn execute<Fn>(&self, task: Fn) -> () where Fn: FnOnce() + Send + 'static {
        let mut task_box = Some(Box::new(task));
        loop {
            for worker in &self.workers {
                match worker.try_lock() {
                    Ok(wk) => {
                        match &wk.work_chann {
                            Some(chann) => {
                                match task_box.take() {
                                    Some(task) => {
                                        chann
                                            .send(task)
                                            .expect("The worker didn't work properly...");
                                    }
                                    _ => {
                                        return;
                                    }
                                }
                                println!("Worker {} received work", wk.id);
                            }
                            _ => (),
                        }
                    }
                    Err(_) => {
                        println!("A worker was busy, continuing");
                        continue;
                    }
                }
            }
        }
    }
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let (tx, rx): (mpsc::Sender<Job>, mpsc::Receiver<Job>) = mpsc::channel();
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            while let Ok(work) = rx.recv() {
                work();
            }
        });
        Worker {
            id,
            thread,
            work_chann: Some(tx),
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers.drain(..) {
            drop(worker.lock().unwrap().work_chann.take());
        }
    }
}
