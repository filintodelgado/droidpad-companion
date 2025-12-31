use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ButtonState {
	Click,
	Release,
	Press,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Direction {
	Up,
	Left,
	Right,
	Down,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum Action {
	Switch { id: String, state: bool },
	Button { id: String, state: ButtonState },
	Dpad { id: String, button: Direction },
	Slider { id: String, value: f32 },
}

impl Action {
	pub fn id(&self) -> &String {
		match self {
			Action::Switch { id, .. } => id,
			Action::Button { id, .. } => id,
			Action::Dpad { id, .. } => id,
			Action::Slider { id, .. } => id,
		}
	}

	pub fn direction(&self) -> Option<&Direction> {
		match self {
			Action::Dpad { button, .. } => Some(button),
			_ => None,
		}
	}

	pub fn value(&self) -> Option<f32> {
		match self {
			Action::Slider { value, .. } => Some(*value),
			_ => None,
		}
	}

	pub fn button_state(&self) -> Option<&ButtonState> {
		match self {
			Self::Button { state, .. } => Some(state),
			_ => None,
		}
	}
}
