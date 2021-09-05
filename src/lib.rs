use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool { // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {}
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     where
    //         F: FnOnce() -> T + Send + 'static,
    //         T: Send + 'static
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} get a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }

        });

        // cannot use this as it will lock longer until unlock (based on the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>>)
        // let thread = thread::spawn(move || {
        //     while let Ok(job) = receiver.lock().unwrap().recv() {
        //         println!("Worker {} got a job; executing.", id);

        //         job();
        //     }
        // });

        Worker { 
            id,
            thread: Some(thread),
        }
    }
}