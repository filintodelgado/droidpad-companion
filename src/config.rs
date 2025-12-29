use std::{collections::{HashMap, hash_map}, fs::File};
use log::{error, info};

use clap::Parser;
use serde::Deserialize;
use resolve_path::PathResolveExt;

use crate::{actions::Action, droidpad::{self, Direction}, exec};

/// Companion tool to run commands controlled by Droidpad.
#[derive(Parser)]
pub struct Arguments {
	/// Address to bind the Websocket server.
	#[arg(short, long, default_value = "0.0.0.0:9123")]
	pub address: String,
	/// YAML configuration file to define the action commands.
	#[arg(short, long, default_value = "~/.config/droidpad-companion.yaml")]
	pub config: String,
}

#[derive(Deserialize)]
pub struct ActionsConfig(HashMap<String, Action>);

impl ActionsConfig {
	pub fn from_file(path: &String) -> Result<Self, Box<dyn std::error::Error>> {
		let path = path.try_resolve()
			.expect("Failed when resolving the path for configuration file");
		info!("Reading the configuration from {}", path.display());
		let file = File::open(path)?;
		let config: Self = serde_yaml::from_reader(file)?;
		Ok(config)
	}
}

const COMMAND_REPLACE_PATTERN: &str = "{}";

impl ActionsConfig {
	pub fn actions(&self) -> &HashMap<String, Action> {
		&self.0
	}
	fn config_action(&self, droidpad_action: &droidpad::Action) -> Option<&Action>{
		if let Some(action) = self.actions().get(droidpad_action.id()) {
			if !action.matches(droidpad_action) {
				error!("The type of the action for {} defined in the configuration file and the one send by droidpad differ: {action:?}, {droidpad_action:?}", droidpad_action.id());
				return None
			}

			return Some(action)
		}

		None
	}

	fn raw_command_for(&self, droidpad_action: &droidpad::Action) -> Option<&String> {
		if let Some(action) = self.config_action(droidpad_action) {
			return match action {
				Action::Button { command } => Some(command),
				Action::Dpad { up, down, left, right } => {
					let direction = droidpad_action.direction()?;
					match direction {
						Direction::Up => up.as_ref(),
						Direction::Left => left.as_ref(),
						Direction::Right => right.as_ref(),
						Direction::Down => down.as_ref()
					}
				},
				Action::Slider { command, .. } => Some(command),
				Action::Switch { command, .. } => Some(command)
			}
		}

		None
	}

	pub fn command_for(&self, droidpad_action: &droidpad::Action) -> Option<String> {
		let command = self.raw_command_for(droidpad_action);

		match droidpad_action.value() {
			Some(value) => command.map(|s| s.replace(COMMAND_REPLACE_PATTERN, value.to_string().as_str())),
			None => command.map(|s| s.into())
		}
	}

	pub fn startup_actions(&self) -> StartupActions<'_> {
		StartupActions { iterator: self.0.iter() }
	}
}

pub struct StartupActions<'a> {
	iterator: hash_map::Iter<'a, String, Action>
}

impl<'a> Iterator for StartupActions<'a> {
	type Item = droidpad::Action;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let (id, action) = match self.iterator.next() {
				Some(v) => v,
				None => return None
			};

			if let Some(startup_command) = action.startup_command() {
				let output = match exec::run_command(startup_command) {
					Ok(o) => o,
					Err(e) => {
						error!("failed to execute startup command for {id}: {e}");
						continue;
					}
				};

				match action.to_droidpad_action(id, &output) {
					Ok(v) => return Some(v),
					Err(e) => {
						error!("Failed to convert action to droidpad json type: {e}");
						continue;
					}
				}
			}
		}
	}
}
