use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

// A trait to use the self inside a dyn Box.
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)();
    }
}

// Trait Object
type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    // Sender end of the mpsc channel. This will be used to send requests to worker threads.
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // We assert if the threadpool size is <= 0. 
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver_share = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver_share)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        for worker in &mut self.workers {
            if let Some(thr) = worker.thread.take() {
                thr.join().unwrap();
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
                         println!("Worker thread: {} got a job, executing", id);
                         job.call_box();
                    },
                Message::Terminate => {
                         println!("Worker thread {} terminated", id);
                         break;
                    },
                }
            }
        });

        Worker { id,
                 thread: Some(thread),
                 }
    }
}
