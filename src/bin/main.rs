//structs, Tcp socket server listening for incoming connections and Tcp stream for reading and writing req, res
use std::net::{TcpListener, TcpStream};

//module, common IO traits
use std::io::prelude::*;

//module, spawn threads
use std::thread;

//module, filesystem manipulation
use std::fs;

//struct, span of time 
use std::time::Duration;

//struct, spawn multiple threads
//Threadpool in lib.rs
use server_threadpool::ThreadPool;

fn main() {

    println!("Type 127.0.0.1:7878 in the browser to connect");
    //define listener for localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //define threadpool with 4 treads
    let pool = ThreadPool::new(8);

    //handle each incoming request in loop

    //take(3) : demonstration purpose : shutdown after 2 requests
        //real-world: shutdown command
    for stream in listener.incoming().take(3) {

        //declare each request as stream, return type Result<TcpStream, panic>
        let stream = stream.unwrap();

        //for each request take closure, give it to thread in pool, run
        pool.execute(|| { 
            //call function that handles requests for each incoming -
            handle_connection(stream);
        });
    }
}

//definine how to respond to http requests
fn handle_connection(mut stream : TcpStream) {
    //define buffersize
    let mut buffer = [0;1024];

    //read incoming requests to buffer
    stream.read(&mut buffer).unwrap();

    //define a GET request for /
    let get = b"GET / HTTP/1.1\r\n";

    //construct slow request (route /sleep)
    let slow = b"GET /sleep HTTP/1.1\r\n";

    //define response for OK and NOT FOUND
    let (status_line, filename) = if buffer.starts_with(get) {
        //request header, filepath to load
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(slow) {
            //construct slow request
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
    } else {
        //if no defined URL
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    //read files 
    let contents = fs::read_to_string(filename).unwrap();

    //define response header and body
    let response = format!(
        //compare content lenght to ensure right content loaded
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line, //header
        contents.len(),
        contents
    );

    //send response as bytes
    stream.write(response.as_bytes()).unwrap();

    //ensure all bytes are transferred
    stream.flush().unwrap();
}

//NOTE: Run $cargo check --> compiler errors
