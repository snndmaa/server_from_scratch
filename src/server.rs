use std::io::Read;
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
                Ok((mut stream, _)) => { // destructuring used here
                    let mut buffer = [0; 1024]; // Dynamically allocate buffer
                    match stream.read(&mut buffer) { // Read data into the buffer    // read requires mutable reference to self as well as &mut [u8]. therefore both array and vector can be used however if an array is used this can cause a buffer overflow if the data received on the socket is more than the specified size.
                        Ok(_) => {
                            println!("Received a Request: {}", String::from_utf8_lossy(&buffer))  // We use this safe version of not checking if the utf8 is valid because we dont want the app to crash even if the ut8 isn't valid just yet, there's validation for this ahead
                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
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