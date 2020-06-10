use log::{error, info};
use std::TcpListener;

fn handle_connection(){

}

fn main() {
    simple_logger::init().unwrap();
    info!("starting");

    let ip = "127.0.0.1:8000";

    let listener = TcpListener.bind(ip).expect("failed to start");
    info!("server started on : {} {}", "http://", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_connection(stream) {
                Ok(_) => (),
                Err(e) => error!("Error handling connection: {}", e),
            },
            Err(e) => error!("Connection failed: {}", e),
        }
    }
    println!("Hello, world!");
}
