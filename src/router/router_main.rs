use core::time;
use std::{net::{TcpListener, TcpStream}, io::Read, fs::File, thread};
use std::io::Write;

use crate::ThreadPool;

pub fn router() {
    let lis = TcpListener::bind("127.0.0.1:7878");
    let lis = match lis {
        Ok(lis) => {
            println!("Start server...");
            lis
        },
        Err(_) => panic!("Cannot bind to server"),
    };

    let pool = ThreadPool::new(4);
    for stream in lis.incoming().take(4) {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream)
                });
                println!("connect stream...");
            }
            Err(e) => {
                panic!("Error stream: {}", e)
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer[..]).unwrap();

    let (status, filename) = routing(&buffer);
    response(&mut stream, status, filename);
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}


fn routing(buffer: &[u8; 1024]) -> (&str, &str) {
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(time::Duration::from_secs(7));
        ("HTTP/1.1 200 OK\r\n\r\n", "static/sleep.html")
    }else {
        ("HTTP/1.1 404 Not found\r\n\r\n", "static/404.html")
    }
}

fn response(stream: &mut TcpStream, status: &str, filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut html = String::new();
    file.read_to_string(&mut html).unwrap();

    let res = format!("{} {}", status, html);

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}