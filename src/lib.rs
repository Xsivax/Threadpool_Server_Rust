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
        //Sender of type Message
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
//docs documentation for section Implementations of struct Threadpool
    //always comment panics

 /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.

    //create new threadpool for type
        //size is number of threads
    pub fn new(size: usize) -> ThreadPool {
        //panic if passed number of threads is 0

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
        ThreadPool { workers, sender }
    }

    /// # Panics
    ///
    /// The `execute` function will panic when the sender fails to send a value over the channel. 

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
            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

//Implement custom Drop Trait called when Threadpool goes out of scope
    //when Threadpool is dropped (in main.rs) threads are joined, loop is exited
impl Drop for ThreadPool {
//NOTE: For some reason doc comments not displayed here, FIX

 /// Implement the Drop Trait.
    ///
    /// Allows to gracefully shut down all threads.
    ///
    /// # Panics
    ///
    /// The `drop` function will panic when the sender fails to send a value over the channel..

    //definition required method drop()
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        //NOTE: Always do seperate tasks in separate loops when working with threads, else correct order not guaranteed !

        //Give Workers signal to Terminate

        //loop through all Workers in Threadpool and send Terminate message
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        //Let all workers finish their Jobs before shutting down 

        for worker in &mut self.workers { //Worker instance in vector
            println!("Shutting down Worker {}", worker.id);

            //if value in Some variant take the value, replace with None and call join()
            if let Some(thread) = worker.thread.take() {

                //join() : block other threads and wait for thread to finish
                //takes ownership of its argument(=execute() in main.rs)
                thread.join().unwrap();
            }   
        }
    }
}

//Define type alias that holds closure execute()
type Job = Box<dyn FnOnce() + Send + 'static>;

//Define Message Enum to be passed from Threadpool to Workers
enum Message {
    //type Job
    NewJob(Job),
    //new type Terminate 
        //to exit and stop loop in Worker
    Terminate,
}

//define internal Worker type
struct Worker {
    id: usize,
    //JoinHandle is return type of thread::spawn
    //Option: can hold Some or None value
        //value can be accessed with Option::take()
    thread: Option<thread::JoinHandle<()>>,
}

//define constructer of new Worker-type
impl Worker {
    //id is counter in for loop above !
    //receiver of data passed through channel declared above !
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker { 
        //thread::spawn returns JoinHandle
            //store JoinHandle in Worker
                //move: ownwnership of receiving end
                //loop: loop as long as server is up 
                //recv(): retrieve message sent over channel (type Job)
                //lock(): aquire Mutex
        let thread = thread::spawn(move || loop {
            //access receiving end in each spawned thread
                //panic if mutex can not be aquired
            let message = receiver.lock().unwrap().recv().unwrap();

            //check if message holds a value and either execute or terminate
            match message {
                //if NewJob variant holds type 
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    //call closure type Job (=execute())
                    job();
                }
                //if Terminate variant
                Message::Terminate => {
                    println!("Worker {} was told to terminate.",id);

                    //exit the loop
                    break;
                }
            }
        }); 

        //return the Worker with id and JoinHandle for thread
        Worker {
            id, 
            thread : Some(thread) 
        }
    }
}

//Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_threadpool() {
        //NOTE: Not sure what to do here, FIX
            //returns Threadpool of Vector of Worker-Instances(id, Message(closure call or break)) and Sender Instance of mpsc::channel(Message(closure call or break)))
            //closure call calls function in main.rs that handles requests and resturns ()
    }
}

//NOTE: unwrap() can always be replaced by error handling Result<T, E>


//NOTE: TO DO: comment panics in Docs

        // # Panics
        // The `new` function will panic if size is zero. +