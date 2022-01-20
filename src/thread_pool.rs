use std::thread::{Thread,JoinHandle, spawn};
use std::sync::mpsc::{Sender,Receiver, channel};
use std::sync::Arc;
use std::sync::Mutex;

/*

ThreadPool implmentation

ThreadPool contains an array with x Workers, defined ahead of time.
The execute method on the ThreadPool sends a Job to the Sender portion of the channel.

Each Worker has a thread.
The workers's thead has an infinite loop that polls the Receiver portion of the channel. 

A Job is a type alias for a function that implements the Send trait.

*/

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Sender<Job>
}

impl ThreadPool {
  pub fn new(size: usize) -> Self {
    assert!(size > 0);

    // We use a channel, the sender is a property of the ThreadPool
    let (sender, receiver) = channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)))
    }

    ThreadPool {
      workers,
      sender
    }
  }

  pub fn execute<F>(&self, f: F)
  where F: FnOnce(),
        F: Send,
        F: 'static 
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}

struct Worker {
  id: usize,
  thread: JoinHandle<()>,
}
impl Worker {
  fn new (id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
    let thread = spawn(move || {
      loop {
        let job = receiver.lock().unwrap().recv().unwrap();
        println!("Worker {} got a job; executing.", id);
        job();
      }
    });
    Worker {
      id,
      thread
    }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;