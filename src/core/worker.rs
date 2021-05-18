use crate::error::AppError;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

pub struct Worker {
    working_running: Arc<AtomicBool>,
    worker_handler: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Just initialize worker, which means it does not start the thread
    pub fn init() -> Self {
        Worker {
            working_running: Arc::new(AtomicBool::new(false)),
            worker_handler: None,
        }
    }

    /// Start the worker thread
    pub fn start<T>(&mut self, job: T) -> Result<(), AppError>
    where
        T: 'static + Send + FnMut() -> (),
    {
        if self.working_running.load(Ordering::SeqCst) || self.worker_handler.is_some() {
            panic!("Worker::start thread is already running");
        }

        info!("Worker::start starting worker thread");
        self.working_running.store(true, Ordering::SeqCst);
        // creating clone to pass into thread
        let alive = self.working_running.clone();
        self.worker_handler = Some(thread::spawn(move || {
            let mut func = job;
            while alive.load(Ordering::SeqCst) {
                func();
            }
        }));

        Ok(())
    }

    /// Close the worker thread
    pub fn stop(&mut self) {
        if !self.working_running.load(Ordering::SeqCst) {
            return;
        }
        self.working_running.store(false, Ordering::SeqCst);
        self.worker_handler
            .take()
            .expect("Worker::stop Called stop on non running thread")
            .join()
            .expect("Worker::stop Could not join spawned thread");
    }
}
