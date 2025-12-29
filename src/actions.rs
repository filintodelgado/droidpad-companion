use serde::Deserialize;

use crate::droidpad;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
	Button { command: String },
	Slider {
		startup: Option<String>,
		command: String
	},
	Dpad {
		up: Option<String>,
		down: Option<String>,
		left: Option<String>,
		right: Option<String>
	},
	Switch {
		startup: Option<String>,
		command: String
	}
}

impl Action {
	pub fn to_droidpad_action(&self, id: &String, value: &String) -> Result<droidpad::Action, Box<dyn std::error::Error>> {
		match self {
			Self::Slider {..} => {
				let value: f32 = value.parse()?;
				Ok(droidpad::Action::Slider { id: id.into(), value: value})
			},
			Self::Switch {..} => {
				let value = if value == "true" { true } else { false };
				Ok(droidpad::Action::Switch { id: id.into(), state: value })
			},
			_ => Err("Can only convert to slider and switch droidpad types".into())
		}
	}

	/// Checks if the action matches with the given droidpad action type wise.
	pub fn matches(&self, other: &droidpad::Action) -> bool {
		matches!(
			(self, other),
			(Self::Button {..}, droidpad::Action::Button {..}) |
			(Self::Dpad {..}, droidpad::Action::Dpad {..}) |
			(Self::Switch {..}, droidpad::Action::Switch {..}) |
			(Self::Slider {..}, droidpad::Action::Slider {..})
		)
	}

	pub fn startup_command(&self) -> Option<&String> {
		match self {
			Self::Slider { startup, .. } => startup.as_ref(),
			Self::Switch { startup, .. } => startup.as_ref(),
			_ => None
		}
	}
}
