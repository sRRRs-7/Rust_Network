use std::{net::{TcpListener, TcpStream}, io::Read, fs::File};
use std::io::Write;

pub fn router() {
    let lis = TcpListener::bind("127.0.0.1:7878");
    let lis = match lis {
        Ok(lis) => {
            println!("Start server...");
            lis
        },
        Err(_) => panic!("Cannot bind to server"),
    };

    for stream in lis.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
                println!("connection established");
            }
            Err(e) => {
                panic!("Error stream: {}", e)
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer[..]).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("index.html").unwrap();
        let mut html = String::new();
        file.read_to_string(&mut html).unwrap();

        let mut f = File::open("index.css").unwrap();
        let mut css = String::new();
        f.read_to_string(&mut css).unwrap();

        let res = format!("HTTP/1.0 200 OK\r\n\r\n {} {}", html, css);

        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let res = "HTTP/1.0 200 OK\r\n\r\n";

        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}