use std::{
  sync::{mpsc::*, Arc, Mutex},
  thread::{self, JoinHandle},
};

pub struct ThreadPool {
  _workers: Vec<Worker>,
  sender: Sender<Job>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    let mut _workers = Vec::with_capacity(size);
    let (sender, receiver) = channel();

    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
      // create threads
      _workers.push(Worker::new(id, Arc::clone(&receiver)))
    }
    ThreadPool { _workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
  _id: usize,
  _thread: JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

impl Worker {
  fn new(_id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
    let _thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker {} got a job; executing.", _id);

      job()
    });
    Worker { _id, _thread }
  }
}
