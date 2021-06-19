use std::thread;

//module, message based concurrency over channels
    //types Sender, Receiver
use std::sync::mpsc;

//struct, thread-safe reference counting smart-pointer
    //for multiple owners of receiver
use std::sync::Arc;

//struct, mutate data from multiple threads, save
    //for ensuring that only one thread accesses and mutates receiver at a time
use std::sync::Mutex;

//build a Threadpool type
pub struct ThreadPool {
    //store workers in vector of Worker-type instances
    workers : Vec<Worker>,
    //declare ThreadPool sending half of channel between threads
        //Sender of type Job
    sender: mpsc::Sender<Job>,
}

//define type alias that holds closure execute() receives
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    //create new threadpool for type
        //size is number of threads
    pub fn new(size: usize) -> ThreadPool {
        //panic if passed number of threads is 0
        //comment panic in Docs
        /// # Panics
        ///The `new` function will panic if size is zero.

        assert!(size > 0);

        //define new channel between Sender and Receiver
        let (sender, receiver) = mpsc::channel();

        //define receiver as a secure Smart Pointer
            //share ownership of mutable receiver among threads
        let receiver = Arc::new(Mutex::new(receiver));

        //initialize a new vector of workers(type = JoinHandle) with number of threads
        let mut workers = Vec::with_capacity(size);

        //for each worker in the vector
        for id in 0..size {
            //create a new Worker instance and push it to vector of workers
                //assign ownership of receiving end of channel to Worker instance
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        //return instance storing vector of workers and sending end of channel
        ThreadPool { workers, sender}
    }

    //define function that executes closure on each thread in the pool
    pub fn execute<F>(&self, f:F)
    where
    //function implements following traits and has lifetime 'static'
        //FnOnce: use closure as argument once, (): closure without params
        //Send: transfer closure between threads
        //'static: lives as long as process
        F: FnOnce() + Send + 'static,
        {
            //store closure in Box pointer
            let job = Box::new(f);

            //send closure from Threadpool to Worker
            self.sender.send(job).unwrap();
        }
}

//define internal Worker type
struct Worker {
    id: usize,
    //JoinHandle is return type of thread::spawn
    thread: thread::JoinHandle<()>,
}

//define constructer of new Worker-type
impl Worker {
    //id is counter in for loop above !
    //receiver of data passed through channel declared above !
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker { 
        //thread::spawn returns JoinHandle
            //store JoinHandle in Worker
                //move: ownwnership of receiving end
                //loop: loop as long as server is up 
                //recv(): retrieve message sent over channel (type Job)
                //lock(): aquire Mutex
        let thread = thread::spawn(move || loop {
            //access receiving end in each spawned thread
                //panic if mutex can not be aquired
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        }); 

        //return the Worker with id and JoinHandle for thread
        Worker {id, thread}
    }
}

//NOTE: unwrap() can always be replaced by error handling Result<T, E>