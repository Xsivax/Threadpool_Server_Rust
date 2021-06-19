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

fn main() {
    //define listener for localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //handle each incoming request in loop
    for stream in listener.incoming() {

        //declare each request as stream, return type Result<TcpStream, panic>
        let stream = stream.unwrap();

        //for each request spawn new thread and call closure
        thread::spawn(|| { 
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

    //define response for Ok and NOT FOUND
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
