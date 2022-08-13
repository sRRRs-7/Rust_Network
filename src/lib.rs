use std::{thread, sync::{mpsc, Arc, Mutex}};


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
pub struct ThreadPool {
    worker: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("process id: {} execute", id);
                job();
            }
        });
        Worker {
            id,
            thread
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut worker = Vec::with_capacity(size);
        for id in 0..size {
            worker.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            worker,
            sender
        }
    }
    pub fn execute<F>(&self, f: F)
        where F :FnOnce() + Send + 'static {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

