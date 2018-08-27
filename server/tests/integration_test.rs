extern crate server;
extern crate mockito;

mod tests {
    use std::net::{IpAddr, Ipv4Addr, TcpStream, TcpListener, SocketAddr};
    use server::ThreadPool;
    use std::io::prelude::*;
    use mockito::mock;
    use std::io::Write;
    use std::io::BufWriter;

    fn connect_and_send_request() {
        let stream = TcpStream::connect("127.0.0.1:7879").unwrap();
        let mut writer = BufWriter::new(stream);
        let s = b"GET / HTTP/1.1\r\n\r\n";
        writer.write(s).unwrap();
        writer.flush().unwrap();
    }

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
        let server_address = [
            SocketAddr::from(([127, 0, 0, 1], 7879)),
        ];

        let listener: TcpListener = TcpListener::bind(&server_address[..]).unwrap();
        let pool = ThreadPool::new(4);

        connect_and_send_request();

        for stream in listener.incoming()  {
            let mut stream = stream.unwrap();

            pool.execute(|| {
                let main_mock = mock("GET", "/").create();
                handle_connection(stream);
                println!("Main mock {}", main_mock);
                main_mock.assert();
            });
        }

        fn handle_connection(mut stream: TcpStream) {
            let mut response = [0; 512];
            stream.read(&mut response).unwrap();
            stream.flush().unwrap();
            println!("Request: {}", String::from_utf8_lossy(&response[..]));
        }
    }

    #[test]
    fn test_sleep_route() {}

    #[test]
    fn test_not_found_route() {}
}
