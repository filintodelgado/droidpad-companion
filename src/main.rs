mod actions;
mod config;
mod droidpad;
mod exec;

use std::{
	error::Error,
	net::{SocketAddr, TcpListener, TcpStream},
	process::exit,
};

use clap::Parser;
use log::{debug, error, info, warn};

use crate::{
	config::{ActionsConfig, Arguments},
	droidpad::ButtonState,
};

fn main() {
	let env = env_logger::Env::default().filter_or("LOG_LEVEL", "info");
	let _ = env_logger::init_from_env(env);

	let arguments = Arguments::parse();
	let address: SocketAddr = match arguments.address.parse() {
		Ok(v) => v,
		Err(_) => {
			error!("IP address \"{}\" is invalid", arguments.address);
			exit(1);
		}
	};

	let socket = match TcpListener::bind(address) {
		Ok(v) => v,
		Err(e) => {
			error!("Failed to bind to \"{}\" ip address: {}", address, e);
			exit(1);
		}
	};

	let config = match ActionsConfig::from_file(&arguments.config) {
		Ok(v) => v,
		Err(e) => {
			error!(
				"Failed to read configuration from \"{}\": {}",
				arguments.config, e
			);
			exit(1)
		}
	};

	info!("Listening on ws://{}/", &arguments.address);

	for stream in socket.incoming() {
		let stream = match stream {
			Ok(s) => s,
			Err(e) => {
				error!("Failed when reading the stream: {e}");
				continue;
			}
		};

		info!("Connected to {}", stream.peer_addr().unwrap());

		if let Err(e) = handle_connection(stream, &config) {
			if let Some(err) = e.downcast_ref::<tungstenite::Error>() {
				match err {
					tungstenite::Error::AlreadyClosed | tungstenite::Error::ConnectionClosed => {
						info!("Disconnected");
						continue;
					}
					_ => {}
				}
			}
			error!("{e}")
		}
	}
}

fn handle_connection(stream: TcpStream, config: &ActionsConfig) -> Result<(), Box<dyn Error>> {
	let mut ws = tungstenite::accept(stream)?;

	for startup_action in config.startup_actions() {
		let payload = startup_action.to_string().into();
		ws.send(tungstenite::Message::Text(payload))?;
	}

	loop {
		let msg = ws.read()?;
		let msg = match msg.into_text() {
			Ok(m) => m,
			Err(e) => {
				error!("Invalid message from peer: {e}");
				continue;
			}
		};

		if msg.is_empty() {
			continue;
		}

		let droidpad_action: droidpad::Action = match msg.parse() {
			Ok(a) => a,
			Err(e) => {
				warn!("Received an invalid droidpad action: {e}");
				continue;
			}
		};

		if let Some(button_state) = droidpad_action.button_state() {
			if let ButtonState::Release = button_state {
				debug!("Ignoring release button state for {}", droidpad_action.id());
				continue;
			}
		}

		match config.command_for(&droidpad_action) {
			Some(command) => {
				debug!("Command for {}: {command}", droidpad_action.id());
				if let Err(e) = exec::run_command(&command) {
					error!("Failed to run command for {}: {e}", droidpad_action.id())
				}
			}
			None => warn!(r#"No command for "{}""#, droidpad_action.id()),
		}
	}
}
