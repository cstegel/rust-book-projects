pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool
    /// size: number of threads in the pool
    /// # Panics
    ///
    /// size < 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}
