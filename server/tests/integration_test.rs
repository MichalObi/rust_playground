extern crate server;
extern crate mockito;

mod tests {
    use std::net::{IpAddr, Ipv4Addr, TcpStream, TcpListener, SocketAddr};
    use server::ThreadPool;
    use mockito::mock;
    use std::io;
    use std::io::prelude::*;
    use std::io::Write;
    use std::io::BufReader;
    use std::io::BufWriter;

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

         let stream = TcpStream::connect("127.0.0.1:7879").unwrap();
         let stream_clone = stream.try_clone().unwrap();

         let mut reader = BufReader::new(stream);
         let mut writer = BufWriter::new(stream_clone);

         let mut s = String::from("\
              GET / HTTP/1.1\r\n\
              Host: 127.0.0.1:7879\r\n\
              \r\n\
          ");

         println!("Request: {}", s);

         let mut response = String::new();
         io::stdin().read_line(&mut s).unwrap();

         writer.write(s.as_bytes()).unwrap();
         writer.flush().unwrap();

         reader.read_line(&mut response).unwrap();
         println!("Response: {}", response.trim());

        for stream in listener.incoming()  {
            let mut stream = stream.unwrap();

            pool.execute(move || {
                handle_connection(stream);
            });
        }

        fn handle_connection(stream: TcpStream) {
            println!("here!");
            let stream_clone = stream.try_clone().unwrap();
            let mut reader = BufReader::new(stream);
            let mut writer = BufWriter::new(stream_clone);

            let main_mock = mock("GET", "/").create();
            let mut s = String::new();

            reader.read_line(&mut s).unwrap();
            writer.write(s.as_bytes()).unwrap();

            writer.flush().unwrap();
            main_mock.assert();
        }
    }

    #[test]
    fn test_sleep_route() {}

    #[test]
    fn test_not_found_route() {}
}
