use std::net::TcpListener;
use std::io::Read;
pub struct Server {
  	addr: String,
}

impl Server {
	pub fn new(addr: String) -> Self {
		Self {
			addr
		}
	}
	pub fn run(self) {
		let listener = TcpListener::bind(&self.addr).unwrap();
		
		println!("Listening on port {}", self.addr);

		loop {
			match listener.accept() {
				Ok((mut stream, client_addr)) => {
					println!("Established connection with {}", client_addr);
					let mut buffer = [0; 1024];
					stream.read(&mut buffer);
				},
				Err(e) => println!("Failed to establish connection - {}", e)
			}
		}
	}
}