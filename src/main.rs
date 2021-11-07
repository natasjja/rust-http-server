fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];
    let string_borrow = &string;
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

    // let ip: String = "127.0.0.1:8080".to_string();

    // let server = Server::new(ip);

    // server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {}
}
