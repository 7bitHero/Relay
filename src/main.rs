// TODO: Disable this after development has come to a suitably advanced state.
#![allow(dead_code)]

#![feature(ip_addr)]
#![feature(convert)]

use std::io;

mod connection_hub;
use connection_hub::ConnectionHub;
mod user;

fn main() {
	let mut connection_hub = ConnectionHub::new().unwrap();

	// The input loop. It takes all sorts of commands registered on this virtual console.
	loop {
		let mut command = String::new();
		io::stdin().read_line(&mut command).unwrap();

		if command.starts_with("msg") {
			let words: Vec<&str> = command.split_whitespace().collect();

			connection_hub.send_message(&words[1], &command).unwrap();
		}
		else if command.starts_with("connect") {
			let words: Vec<&str> = command.split_whitespace().collect();

			if words.len() == 3 {
				let host = format!("{}:{}", words[1], words[2]);

				println!("Connecting to {}", &host);
				connection_hub.connect(&host.as_str()).unwrap();
			}
			else {
				println!("The command 'connect' takes exactly two arguments, in the form of:\nconnect <hostname> <address>");
			}
		}
		else if command.starts_with("exit") || command.starts_with("quit") {
			break;
		}
		else {
			println!("No such command is registered. Please look up the command table to find out more.");
		}
	}
}

#[cfg(test)]
mod tests {
	use std::io::prelude::*;
	use std::net::{TcpListener, TcpStream};
	use std::thread;

	static ANSWER: [u8; 1] = [42];

	#[test]
	fn listen() {
		let listener = TcpListener::bind("*:25565").unwrap();

		let mut client = listener.accept().unwrap();
		let transferred = client.0.write(&ANSWER).unwrap();

		assert_eq!(transferred, 1);
	}

	fn connect(hostname: &str) {
		let mut stream = TcpStream::connect(hostname).unwrap();

		let mut answer = [0; 1];
		let transferred = stream.read(&mut answer).unwrap();

		assert_eq!(answer, ANSWER);
		assert_eq!(transferred, 1);
	}

	#[test]
	fn connect_local() {
		connect("127.0.0.1:25565");
	}

	#[test]
	fn connect_remote() {
		connect("deydoo.servebeer.com:25565");
	}
}
