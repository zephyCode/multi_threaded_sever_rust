use std::{
    sync::{Arc, Mutex, mpsc},
    thread
};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    _sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // Size of thread 0 or smaller than 0 our thread will panic.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (_sender, _receiver) = mpsc::channel();
        let _receiver = Arc::new(Mutex::new(_receiver));
        let mut _workers = Vec::with_capacity(size);
        for id in 0..size {
            _workers.push(Worker::new(id, Arc::clone(&_receiver)));
        }
        ThreadPool { _workers, _sender }
    }

    pub fn execute<F> (&self, _f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(_f);
        self._sender.send(job).unwrap();
    }
}

struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(_id: usize, _reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let _thread = thread::spawn(move || {
            loop {
                let job = _reciever.lock().unwrap().recv().unwrap();
                println!("Worker {_id} got a job; executing.");
                job();
            }
        });
        Worker { _id, _thread }
    }
}