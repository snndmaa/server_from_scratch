fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {    // Main Constructor. Also Self and Server are interchangeable
        Self {
            addr
        }
    }

    fn run(self) {  // run takes ownership of entire struct(self variable that turns to the struct)
        println!("Listening on {}", self.addr)
    }
}