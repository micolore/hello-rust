use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use ws::ThreadPool;
fn main() {
    println!("Hello, rust web-server!");

    let listener = TcpListener::bind("127.0.0.1:7788").unwrap();
    for stream in listener.incoming() {

       let pool = ThreadPool::new(4);
        let _stream = stream.unwrap();

        println!("rust web-server connection eatablished");

        pool.execute(||{
            single_thread_handle_conn(_stream);
        });
    }
}

fn single_thread_handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // if buffer.starts_with(get){
    //     let contents =fs::read_to_string("hello.html").unwrap();
    //     let response =format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }else{
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let response = format!("{}{}", status_line, contents);
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    let (status_line, file) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // sleep 2s
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(file).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
