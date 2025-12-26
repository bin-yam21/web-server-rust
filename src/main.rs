use std::fs;
use std::io::Read;
// import the tcp listener struct from the std lib
use std::net::TcpListener;
// import the tcp stream struct from the std lib
use std::net::TcpStream;
use std::io::prelude::*;
fn main() {
    // create listener and bind to localhost and port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // we loop through the connections and call the incoming method on the listener 
    // which is going to give us an iterator over the connections being received
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // we made the string argument mutable because the read method on stream takes mutable
    // reference to self
    // create the buffer to hold the data that is read
    let mut buffer: [u8; 1024] = [0; 1024];
    
    // read the stream and store it in the buffer
    stream.read(&mut buffer).unwrap();

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";

    let (status_line , file_name) = if buffer.starts_with(get) {

      ("HTTP/1.1 200 OK" , "index.html")
    
        
    } else {
     ("HTTP/1.1 404 NOT FOUND" , "404.html")
    };


     let contents = fs::read_to_string(file_name).unwrap();

     let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
     status_line,
    contents.len(),
    contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

}