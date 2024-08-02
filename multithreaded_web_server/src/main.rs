use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Server online.");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("src/hello.html").unwrap();
        let length = content.len();

        let response = format!(
            "{status}\r\n\
            Content-Length: {length}\r\n\r\n\
            {content}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        println!("Non-root/non-GET request!");
    }
}
