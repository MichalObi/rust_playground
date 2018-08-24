extern crate server;
extern crate mockito;

mod tests {
    use std::io::prelude::*;
    use std::net::{IpAddr, Ipv4Addr, TcpStream, TcpListener};
    use server::ThreadPool;
    use mockito::mock;

    #[test]
    fn test_server_setup() {
        let server_address = "127.0.0.1:7878";
        let listener: TcpListener = TcpListener::bind(server_address).unwrap();
        let stream: TcpStream = TcpStream::connect(server_address).unwrap();

        assert_eq!(stream.local_addr().unwrap().ip(),
                   IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        assert_eq!(listener.local_addr().unwrap().ip(),
                   IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }

    #[test]
    fn test_main_route() {
        let server_address = "127.0.0.1:7879";
        let listener: TcpListener = TcpListener::bind(server_address).unwrap();
        let pool = ThreadPool::new(4);

        TcpStream::connect(server_address).unwrap();

        for stream in listener.incoming()  {
            let mut stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        }

        fn handle_connection(mut stream:TcpStream) {
            let main_mock = mock("GET", "/").create();
            let main_route_get = "GET / HTTP/1.0\r\nHost: 127.0.0.1:7878\r\n\r\n";

            stream.flush().unwrap();

            let mut response = String::new();

            stream.read_to_string(&mut response).unwrap();
            stream.write(main_route_get.as_bytes()).unwrap();

            println!("Response: {}", response);
            main_mock.assert();
        }
    }

    #[test]
    fn test_sleep_route() {}

    #[test]
    fn test_not_found_route() {}
}
