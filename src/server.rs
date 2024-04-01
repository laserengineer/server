use std::io::Read;
use std::net::TcpListener;

// Rust use one block of code hold data for struct
pub struct Server {
    addr: String,
}

// Rust use one block of code implement the struct and interface
impl Server {
    // Method needs an instance of the struct to implement, accept the first parameter self
    // '&self' does not take the ownership of the whole struct, only take the reference.

    // Self == Sever
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // Function does not need an instance of the interface to implement
    // without '&' run method will the ownership of entire struct
    pub fn run(self) {
        println!("Server listening on: {}", self.addr);
        // let listener = TcpListener::bind(&self.addr).unwrap();
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    };
                    // stream.read();
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
