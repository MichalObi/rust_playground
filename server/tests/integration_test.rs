extern crate server;
extern crate mockito;

use std::net::{IpAddr, Ipv4Addr, TcpStream, TcpListener};


use server::ThreadPool;
use mockito::mock;

#[test]
fn test_server_connection() {
    let server_address = "127.0.0.1:7878";
    let listener = TcpListener::bind(server_address).unwrap();

    let stream = TcpStream::connect(server_address)
                           .expect("Couldn't connect to the server...");

    assert_eq!(stream.local_addr().unwrap().ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    assert_eq!(listener.local_addr().unwrap().ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
}

#[test]
fn test_main_route() {}

#[test]
fn test_sleep_route() {}

#[test]
fn test_not_found_route() {}
