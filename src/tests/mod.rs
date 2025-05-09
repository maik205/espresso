use std::{ sync::{ Arc, Mutex }, thread, time::Duration };

use crate::threads::{ pigeonhole_threads, stream_threads, TPool };
#[test]
pub fn thread_pool_should_process_asynchronously() {
    let pool = pigeonhole_threads::ThreadPool::new(2);
    let result = Arc::new(Mutex::new(0));
    let t1 = Arc::clone(&result);
    let t2 = Arc::clone(&result);
    pool.exec(move || {
        thread::sleep(Duration::from_millis(2000));
        *t1.lock().unwrap() += 1;
    });
    pool.exec(move || {
        thread::sleep(Duration::from_millis(1000));
        *t2.lock().unwrap() += 1;
    });
    thread::sleep(Duration::from_millis(2110));
    assert!(*result.lock().unwrap() == 2);
}

#[test]
pub fn thread_pool_should_be_decently_performant() {
    let pool = pigeonhole_threads::ThreadPool::new(100);
    let result = Arc::new(Mutex::new(0));
    for _ in 0..1000000 {
        let t = Arc::clone(&result);
        pool.exec(move || {
            *t.lock().unwrap() += 1;
        });
    }
    assert!(*result.lock().unwrap() == 1000000);
}

#[test]
pub fn thread_pool_should_process_asynchronously_ws() {
    let pool = stream_threads::ThreadPool::new(2);
    let result = Arc::new(Mutex::new(0));
    let t1 = Arc::clone(&result);
    let t2 = Arc::clone(&result);
    pool.exec(move || {
        thread::sleep(Duration::from_millis(50));
        *t1.lock().unwrap() += 1;
    });
    pool.exec(move || {
        thread::sleep(Duration::from_millis(100));
        *t2.lock().unwrap() += 1;
    });
    thread::sleep(Duration::from_millis(110));
    assert!(*result.lock().unwrap() == 2);
}
