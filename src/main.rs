use log::{error, info};
use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{fmt, fs};
use std::io::{Read, Write};

struct Request<'a> {
    method: &'a str,
    uri: &'a Path,
    httpVersion: &'a str,
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.method,
            self.uri.display(),
            self.httpVersion,
        )
    }
}

fn parse_line(req: &str) -> Result<Request, Box<dyn Error>> {
    let mut parts = req.split_whitespace();
    let method = parts.next().ok_or("no method specified")?;
    if method != "GET" {
        Err("not supported")?;
    }

    let uri = Path::new(parts.next().ok_or("no uri specified")?);
    let normUri = uri.to_str().expect("unicode is invalid");
    const root: &str = "/";
    if !Path::new(&format!("{}", normUri)).exists() {
        Err("resource does not exist")?;
    }

    let httpVersion = parts.next().ok_or("no version specified")?;
    if httpVersion != "HTTP/1.1" {
        Err("ONLY HTTP 1.1 MAN")?;
    }

    Ok(Request {
        method,
        uri,
        httpVersion,
    })
}

fn onConnection(mut tcp_stream: TcpStream) -> Result<(), Box<dyn Error>>{
    let mut buffer = [0; 512];
    tcp_stream.read(&mut buffer).unwrap(); //read stream data onto buffer
    let req = String::from_utf8_lossy(&buffer[..]);
    let line = req.lines().next().unwrap();

    match parse_line(&line) {
        Ok(req) => {
            info!("Request: {}", &req);

            let contents = fs::read_to_string("index.html").unwrap();
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);

            tcp_stream.write(response.as_bytes()).unwrap();
            tcp_stream.flush().unwrap();
        }
        Err(e) => error!("not formartted: {}", e),
    }

    Ok(())
}

fn main() {
    simple_logger::init().unwrap();
    info!("starting");

    let ip = "127.0.0.1:8000";

    let listener = TcpListener::bind(ip).expect("failed to start");
    info!("server started on : {} {}", "http://", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match onConnection(stream) {
                Ok(_) => (),
                Err(e) => error!("Error handling connection: {}", e),
            },
            Err(e) => error!("Connection failed: {}", e),
        }
    }
    println!("Hello, world!");
}
