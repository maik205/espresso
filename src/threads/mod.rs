type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod pigeonhole_threads;
pub mod stream_threads;

pub trait TPool {
    fn new(size: usize) -> Self;
    fn exec<Fn>(&self, task: Fn) -> () where Fn: FnOnce() + Send + 'static;
}
