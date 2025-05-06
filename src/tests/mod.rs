use std::{ sync::{ Arc, Mutex }, thread, time::Duration };

use crate::threads::ThreadPool;

#[test]
pub fn thread_pool_should_process_asynchronously() {
    let pool = ThreadPool::new(2);
    let result = Arc::new(Mutex::new(0));
    let t1 = Arc::clone(&result);
    let t2 = Arc::clone(&result);
    pool.execute(move || {
        thread::sleep(Duration::from_millis(500));
        *t1.lock().unwrap() += 1;
    });
    pool.execute(move || {
        thread::sleep(Duration::from_millis(1000));
        *t2.lock().unwrap() += 1;
    });
    thread::sleep(Duration::from_millis(1010));
    assert!(*result.lock().unwrap() == 2);
}

#[test]
pub fn thread_pool_should_be_decently_performant() {
    let pool = ThreadPool::new(100);
    let result = Arc::new(Mutex::new(0));
    for i in 0..1000000 {
        let t = Arc::clone(&result);
        pool.execute(move || {
            *t.lock().unwrap() += 1;
        });
    }
}
