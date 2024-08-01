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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("src/hello.html").unwrap();
    let length = content.len();

    let response = format!(
        "{status}\r\n\
        Content-Length: {length}\r\n\r\n\
        {content}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
