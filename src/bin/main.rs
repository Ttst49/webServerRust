use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use webserverrust::TaskGroup;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    let group = TaskGroup::new(4);

    for flow in listener.incoming() {
        let flow = flow.unwrap();

        thread::spawn(||{
            manage_connection(flow)

        });
    }
}

fn manage_connection(mut flow:TcpStream){
    let mut buffer = [0;1024];
    flow.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let pause = b"GET /pause HTTP/1.1\r\n";

    let (line_status, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./template/index.html")
    }else if buffer.starts_with(pause) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./template/index.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND", "./template/404.html")
    };

        let content = fs::read_to_string(file_name).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            line_status,
            content.len(),
            content
        );

        flow.write(response.as_bytes()).unwrap();
        flow.flush().unwrap();
}

