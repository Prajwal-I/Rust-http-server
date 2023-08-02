use std::net::TcpListener;
use crate::Request;
use std::convert::TryFrom;
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
					println!("buffer size - {}",buffer.len());

					match stream.read(&mut buffer) {
						Ok(_) => {
							println!("Recieved request - \n{}", String::from_utf8_lossy(&buffer));
							match Request::try_from(&buffer[..]) {
								Ok(request) => {},
								Err(e) => println!("Failed to parse request - {}", e),
							}
						},
						Err(e) => println!("Failed to read data from connection - {}", e)
					}
				},
				Err(e) => println!("Failed to establish connection - {}", e)
			}
		}
	}
}
