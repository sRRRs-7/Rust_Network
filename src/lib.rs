use std::{thread, sync::{mpsc, Arc, Mutex}};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

enum Message {
    NewJob(Job),
    Terminate
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("--Sending terminate message--");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("--Shut down all thread--");
        for worker in &mut self.workers {
            println!("Shut down thread {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("worker {} get a job", id);
                        job()
                    },
                    Message::Terminate => {
                        println!("terminate: worker: {}", id);
                        break
                    }
                };
            }
        });
        Worker {
            id,
            thread: Some(thread)
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        };

        ThreadPool {
            workers,
            sender
        }
    }
    pub fn execute<F>(&self, f: F)
        where F :FnOnce() + Send + 'static {
            let job = Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

