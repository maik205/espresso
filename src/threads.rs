use std::{ sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex }, thread::{ self, JoinHandle } };

/// ## Info
/// The thread pool holds a number of threads to process web requests concurrently
/// ## Panics
/// If the number of threads is <= 0
pub struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
    pub busy: Arc<Mutex<bool>>,
    work_chann: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
}
/// A worker-state based thread pool implementation
/// ## Pros:
/// - Health-checkable (% of workers busy) for scaling and sharding if necessary
/// ## Cons:
/// Checks incur too much runtime cost due to high Mutex usage.
impl ThreadPool {
    pub fn new(threads_num: usize) -> ThreadPool {
        let mut workers: Vec<Worker> = vec![];
        assert!(threads_num > 0);
        for i in 0..threads_num {
            workers.push(Worker::new(i));
        }
        println!("Created {threads_num} workers");
        ThreadPool { workers: workers }
    }

    pub fn execute<Fn>(&self, task: Fn) -> () where Fn: FnOnce() + Send + 'static {
        let task_box = Box::new(task);
        loop {
            for (i, worker) in self.workers.iter().enumerate() {
                let is_busy = {
                    let is_worker_busy_arc = Arc::clone(&worker.busy);
                    let is_worker_busy = is_worker_busy_arc.lock().unwrap();

                    *is_worker_busy
                };
                if is_busy == false {
                    // Wrap the lock in a scope to prevent permanent locking.
                    {
                        // Mark the worker busy and send work to process.
                        let is_worker_busy_arc = Arc::clone(&worker.busy);
                        let mut is_worker_busy = is_worker_busy_arc.lock().unwrap();
                        *is_worker_busy = true;
                    }
                    worker.work_chann.send(task_box).expect("The worker didn't work properly...");
                    return;
                } else {
                    // println!("worker {i} busy...finding next worker");
                }
            }
            // println!("Looping through workers");
        }
    }
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let (tx, rx): (
            mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
            mpsc::Receiver<Box<dyn FnOnce() + Send + 'static>>,
        ) = mpsc::channel();
        let busy_arc: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let worker_busy_arc: Arc<Mutex<bool>> = Arc::clone(&busy_arc);
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            while let Ok(work) = rx.recv() {
                println!("Working");
                work();
                {
                    let mut busy: std::sync::MutexGuard<'_, bool> = worker_busy_arc.lock().unwrap();
                    *busy = false;
                }
                println!("Finished");
            }
        });
        Worker {
            id,
            thread,
            busy: Arc::clone(&busy_arc),
            work_chann: tx,
        }
    }
}

pub struct ThreadPoolWorkStealing {
    workers: Vec<WorkerWS>,
    work_sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
}
impl ThreadPoolWorkStealing {
    pub fn new(size: usize) -> ThreadPoolWorkStealing {
        let (tx, rx) = mpsc::channel();
        let work_receiver: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send + 'static>>>> = Arc::new(
            Mutex::new(rx)
        );
        let mut workers: Vec<WorkerWS> = Vec::new();
        for i in 0..size {
            workers.push(WorkerWS::new(i, &work_receiver));
        }
        ThreadPoolWorkStealing { workers, work_sender: tx }
    }
    pub fn execute<Fn>(&self, work: Fn) where Fn: FnOnce() + Send + 'static {
        self.work_sender
            .send(Box::new(work))
            .expect("Work was not sent to the common work channel.");
    }
}
struct WorkerWS {
    id: usize,
    thread: JoinHandle<()>,
}
impl WorkerWS {
    pub fn new(
        id: usize,
        recv: &Arc<Mutex<Receiver<Box<dyn FnOnce() + Send + 'static>>>>
    ) -> WorkerWS {
        let binding = Arc::clone(recv);

        let thread = thread::spawn(move || {
            let work_stream = binding.lock().unwrap();
            while let Ok(work) = work_stream.recv() {
                println!("Thread {id} received work");
                work();
            }
        });

        WorkerWS { id, thread }
    }
}
