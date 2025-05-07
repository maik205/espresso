type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod pigeonhole_threads;
pub mod stream_threads;
