/*
 * The connection hub class is there to manage incoming connections, aswell as establishing new
 * connections. It is the central hub to communicate with specific clients.
 */

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr, ToSocketAddrs, Ipv4Addr, IpAddr};
use std;

use std::sync::{Arc, Mutex};
use std::thread;

use std::collections::HashMap;
use user::User;

type SafeUserMap = Arc<Mutex<HashMap<SocketAddr, User>>>;

pub struct ConnectionHub {
	listener: TcpListener,
	users: SafeUserMap
}

impl ConnectionHub {
	pub fn new() -> Result<ConnectionHub, String> {
		// Try to find an open port, starting with a magic standard port.
		let listener = match ConnectionHub::try_binding() {
			Ok(listener) => listener,
			Err(_) => return Err(format!("Could not find an open port."))
		};

		let connection_hub = ConnectionHub {
			listener: listener,
			users: Arc::new(Mutex::new(HashMap::new()))
		};

		let listener_clone = connection_hub.listener.try_clone().unwrap();
		let users_clone = connection_hub.users.clone();
		thread::spawn(move || {
			ConnectionHub::accept_connections(listener_clone, users_clone);
		});

		Ok(connection_hub)
	}

	/// Try to listen to a port from the magic number 44942. In case this function finds an open
	/// port it opens a listener on that port. In case it cannot find an open port for whatever
	/// reasons it returns an error string.
	fn try_binding() -> Result<TcpListener, ()> {
		for port in 44943..std::u16::MAX {
			match TcpListener::bind(("127.0.0.1", port)) {
				Ok(listener) => {
					println!("Bound to port {}", port);
					return Ok(listener);
				},
				_ => {/*keep looking*/}
			}
		}

		// TODO: Return an error code so as to better narrow down the error.
		Err(())
	}

	fn accept_connections(listener: TcpListener, users: SafeUserMap) {
		for stream in listener.incoming() {
			let user = User::new(stream.unwrap());

			let mut user_map = users.lock().unwrap();
			user_map.insert(user.remote_address(), user);
		}

		drop(listener);
	}

	pub fn connect<A: ToSocketAddrs>(&mut self, address: &A) -> Result<(), String> {
		let stream = match TcpStream::connect(address) {
			Ok(stream) => stream,
			Err(err) => return Err(format!("Could not connect to user. {}", err))
		};

		let user = User::new(stream);
		let mut user_map = self.users.lock().unwrap();

		user_map.insert(user.remote_address(), user);
		Ok(())
	}

	// TODO: Instead of a function that simply calls another function, it would be nice to get the
	// client back. However at the moment I am just too tired to fight the borrow-checker, sorry.
	pub fn send_message<A: ToSocketAddrs>(&mut self, addr: &A, msg: &str) -> Result<usize, String> {
		// Most beautiful. (*cough* TODO *cough*)
		let mut user_map = self.users.lock().unwrap();
		let mut user = user_map.get_mut(/*&addr.to_socket_addrs().unwrap().nth(0).unwrap()*/&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 44943)).unwrap();

		user.send_message(msg)
	}
}
