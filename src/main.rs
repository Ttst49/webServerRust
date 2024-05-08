use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:9999").unwrap();

    for flow in listener.incoming() {
        let flow = flow.unwrap();

        manage_connection(flow)
    }
}

fn manage_connection(mut flow:TcpStream){
    let mut buffer = [0;1024];
    flow.read(&mut buffer).unwrap();

    let content = fs::read_to_string("./src/index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    flow.write(response.as_bytes()).unwrap();
    flow.flush().unwrap();

}
