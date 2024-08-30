use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use multithreaded_web_server::ThreadPool;

fn main() {
    let (shutdown_tx, shutdown_rx) = mpsc::channel(); // Channel to signal shutdown
    let pool = Arc::new(Mutex::new(ThreadPool::new(4)));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server online.");

    let shutdown_flag = Arc::new(Mutex::new(false));

    for stream in listener.incoming() {
        let shutdown_flag_clone = Arc::clone(&shutdown_flag);
        let pool_clone = Arc::clone(&pool);
        let shutdown_tx_clone = shutdown_tx.clone();

        if *shutdown_flag.lock().unwrap() {
            break;
        }

        let stream = stream.unwrap();

        pool_clone.lock().unwrap().execute(move || {
            let should_shutdown = handle_connection(stream);

            if should_shutdown {
                let mut shutdown = shutdown_flag_clone.lock().unwrap();
                *shutdown = true;
                println!("Shutdown initiated.");

                // Notify the main thread to handle shutdown
                shutdown_tx_clone.send(()).unwrap();
            }
        });
    }

    // Wait for the shutdown signal
    shutdown_rx.recv().unwrap();
    println!("Shutdown signal received. Dropping thread pool.");

    // Drop the thread pool
    drop(pool);

    println!("Server shutting down.");
}

fn handle_connection(mut stream: TcpStream) -> bool {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /shutdown HTTP/1.1" => ("HTTP/1.1 503 Service Unavailable", "503.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();

    let response = format!(
        "{status}\r\n\
        Content-Length: {length}\r\n\r\n\
        {content}"
    );

    stream.write_all(response.as_bytes()).unwrap();

    // Return true if the server should shut down
    status == "HTTP/1.1 503 Service Unavailable"
}
