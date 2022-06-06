use std::io::Result;
use console::{Term, Key};
use crate::{Action, Action::*};


// Logic for user input in stdin
pub fn handle_keypress(term: &Term) -> Result<Action> {
    // See https://docs.rs/console/0.15.0/console/enum.Key.html
    // for a full list of available keys that can be implemented here
    match term.read_key()? {
	Key::Char('\u{11}') => { // C-q
	    return Ok(Quit)
	},
	Key::Char('\u{10}') => { // C-p
	    return Ok(MoveUp)
	},
	Key::Char('\u{e}') => { // C-n
	    return Ok(MoveDown)
	}
	Key::Char('\u{2}') => { // C-b
	    return Ok(MoveLeft)
	}
	Key::Char('\u{6}') => { // C-f
	    return Ok(MoveRight)
	}	
	Key::Char('\u{13}') => { // C-s
	    return Ok(Save)
	},
	Key::Char('\u{c}') => { // C-l
	    return Ok(SetActiveFilePath)
	}
	Key::Char('\0') => { // ` (i make this char with Shift-~)
	    return Ok(PrintMini)
	}
	k => { // Anything else
	    println!("DEBUG: Read key {:?}",k);
	    return Ok(DoNo)
	},
    };
}
