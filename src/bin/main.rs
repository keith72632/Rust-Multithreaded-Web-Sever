use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::{thread, time::*};
use server::ThreadPool;

macro_rules! log {
    ($msg: literal) => {
        println!("{}", stringify!(&msg));
    };
}
fn main() {
    log!("hey");
    // let mut connections = 0;
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        // connections+=1;
        let stream = stream.unwrap();
        // println!("Connection {}", connections);

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];

    // Reads contents into buffer as byte code
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status, file_name) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    // File handlers
    let contents = fs::read_to_string(file_name).unwrap();

    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // );

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

/* 
---Response Structure---
1. HTTP-Version Status-code Reason-Phrase CRLF
2. headers CRLF
3. message-body
ex: HTT/1.1 200 OK\r\n\r\n
*/