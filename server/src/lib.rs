use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {

    /// Create a brand new ThreadPool.
    ///
    /// # Panics
    ///
    /// if size is 0 fn new will panic!

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
             workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    /// Execute closure send to worker.
    ///
    /// Take closure as new job, also send proper msg to worker.

    pub fn execute<F>(&self, f:F)
     where F:FnOnce() + Send + 'static
     {
         let job = Box::new(f);

         self.sender.send(Message::NewJob(job)).unwrap();
     }
}

/// Run code when ThreadPool go out of scope e.g. program stopped.

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
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
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn threadpool_size() {
        use ThreadPool;
        let test_threadpool_size = 4;
        let pool = ThreadPool::new(test_threadpool_size);
        let threadpool_workers_size = pool.workers.len();

        println!("ThreadPool with test size {} has {} workers.",
        test_threadpool_size, threadpool_workers_size);
        assert_eq!(test_threadpool_size, threadpool_workers_size);
    }

    /// Test if threadpool closure execution will change variable value on execution

    #[test]
    fn threadpool_execute() {
        fn test_fn(test_msg: &mut bool) -> &bool {
            *test_msg = true;
            return test_msg;
        }

        use ThreadPool;
        let test_threadpool_size = 4;
        let pool = ThreadPool::new(test_threadpool_size);
        let mut was_test_fn_executed = false;

        println!("Value BEFORE execution: {}", was_test_fn_executed);

        pool.execute(move || {
            test_fn(&mut was_test_fn_executed);
            println!("Value AFTER execution: {}", was_test_fn_executed);
            assert_eq!(was_test_fn_executed, true);
        });
    }
}
