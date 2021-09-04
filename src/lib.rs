pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     where
    //         F: FnOnce() -> T + Send + 'static,
    //         T: Send + 'static
}