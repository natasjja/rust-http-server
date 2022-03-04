use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // 1024 bytes enough for this small server
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("{:?}", request);
                                }
                                Err(e) => println!("Failed to pass request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
