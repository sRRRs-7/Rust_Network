use std::net::{TcpListener, TcpStream};

pub fn router() {
    let lis = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in lis.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                panic!("{}", e)
             }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    println!("request: {:?}", stream);
}