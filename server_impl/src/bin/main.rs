extern crate server_impl;

use server_impl::ThreadPool; 
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::thread; 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let mut tick = 0;
 
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {handle_connection(stream);});
        if tick >= 5 {
            break;
        }
        tick += 1;
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    //we have to set stream as mutable since it is an instance of TcpStream which can read more data than requested, thus resulting in updating internal state
    //on every read.
    stream.read(&mut buffer).unwrap();

    // from_utf8_lossy - takes input of &[u8]..lossy inserts replacement_char if an invalid utf8 sequence is encountered
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get_request = b"GET / HTTP/1.1\r\n";
    let sleep_request = b"GET /sleep HTTP/1.1\r\n";

    let (status_response, file_name) = if buffer.starts_with(get_request) {
        ("HTTP/1.1 200 Ok\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep_request) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 404 Not Found\r\n\r\n", "404.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    //Http response format :
    //Http-version Code Response-description CRLF
    //Headers CRLF
    //Message
    let response = format!("{}{}", status_response, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
