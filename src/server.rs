use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {    // Main Constructor. Also Self and Server are interchangeable
        Self {
            addr
        }
    }

    pub fn run(self) {  // run takes ownership of entire struct(self variable that turns to the struct)
        println!("Listening on {}", self.addr);

        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap(); //unwrap returns tcp listener on success and exits with err on fail

        loop {
            match listener.accept() {
                Ok((stream, addr)) => { // destructuring used here
                    let a = 5;
                    println!("Ok")
                },
                Err(e) => println!("Failed to establish connection: {}", e)
            }

            let res: Result<(std::net::TcpStream, std::net::SocketAddr), std::io::Error> = listener.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}