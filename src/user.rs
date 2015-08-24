/*
 * The user represents a structure that can receive data from  or send data to a certain user that
 * the program is currently connected to.
 */

use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};
use std::thread;

pub struct User {
	stream: TcpStream,
}

impl User {
	pub fn new(stream: TcpStream) -> User {
		let user = User {
			stream: stream,
		};

		let mut receive_stream = user.stream.try_clone().unwrap();
		thread::spawn(move || {
			User::receive_messages(&mut receive_stream);
		});

		user
	}

	fn receive_messages(stream: &mut TcpStream) {
		loop {
			let mut data = vec![0; 128];

			let size = match stream.read(&mut data) {
				Ok(size) => size,
				Err(err) => {
					println!("{:?} has been closed due to: {}", stream, err);
					break
				}
			};

			if size == 0 {
				println!("{} disconnected.", stream.peer_addr().unwrap());
				break
			}

			let message = String::from_utf8(data).unwrap();
			println!("Message received from '{}':", stream.peer_addr().unwrap());
			println!("{}", message);
		}

		drop(stream);
	}

	pub fn send_message(&mut self, msg: &str) -> Result<usize, String> {
		let data = msg.as_bytes();

		// TODO: This is a stupid workaround, because the io::error::Error type is private.
		match self.stream.write(&data) {
			Ok(size) => Ok(size),
			Err(err) => Err(format!("{}", err))
		}
	}

	pub fn remote_address(&self) -> SocketAddr {
		self.stream.peer_addr().unwrap()
	}
}
