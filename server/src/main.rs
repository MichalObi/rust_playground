use std::net::TcpListener;

fn main() {
    // unwrap terminate if errors - tmp solution
    let listener = TcpListener::bind("127.0.0.1:1212").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Conntection established!");
    }
}
