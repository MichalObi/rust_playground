extern crate server;

mod tests {
    use std::net::{IpAddr, Ipv4Addr, TcpStream, TcpListener, SocketAddr};
    use server::ThreadPool;
    use std::io::prelude::*;
    use std::io::Write;
    use std::io::BufWriter;
    use std::mem::drop;

    fn connect_and_send_request(route: &'static str) {
        let stream = TcpStream::connect("127.0.0.1:7879").unwrap();
        let mut writer = BufWriter::new(stream);

        let s;

        if route == "main" {
            s = "GET / HTTP/1.1\r\n\r\n";
        } else if route == "sleep" {
            s = "GET /sleep HTTP/1.1\r\n\r\n";
        } else {
            s = "GET /xxx HTTP/1.1\r\n\r\n";
        }

        writer.write(s.as_bytes()).unwrap();
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

        let req_route: &'static str = "main";

        connect_and_send_request(req_route);

        for stream in listener.incoming()  {
            let mut stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        }

        fn handle_connection(mut stream: TcpStream) {
            let get = b"GET / HTTP/1.1\r\n";
            let mut buffer = [0; 512];

            stream.read(&mut buffer).unwrap();
            let result = assert_eq!(buffer.starts_with(get), true);
            drop(result);
        }
    }

    #[test]
    fn test_sleep_route() {}

    #[test]
    fn test_not_found_route() {}
}
