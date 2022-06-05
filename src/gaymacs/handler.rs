use std::io::Result;
use console::{Term, Key};
use crate::{Action, Action::*};

// Logic for user input in stdin
pub fn handle_keypress(term: &Term) -> Result<Action> {
    let mut out = DoNo;
    // See https://docs.rs/console/0.15.0/console/enum.Key.html
    // for a full list of available keys that can be implemented here
    match term.read_key()? {
	Key::Char('q') => {
	    return Ok(Quit)
	},
	Key::Char('p') => {
	    return Ok(MoveUp)
	},
	Key::Char('n') => {
	    return Ok(MoveDown)
	}
	Key::Char('b') => {
	    return Ok(MoveLeft)
	}
	Key::Char('f') => {
	    return Ok(MoveRight)
	}	
	Key::Enter => {
	    return Ok(Save)
	},
	Key::Char('m') => {
	    return Ok(PrintMini)
	}
	_ => return Ok(DoNo),
    };
}
