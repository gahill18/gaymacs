use std::io::Result;
use std::collections::HashMap;
use console::{Term, Key};

use crate::{Action, gaymacs::mini::MiniBuf, Frame,};
use crate::{Action::*,};
// use crate::gaymacs::mini::MiniBuf;

// The Handler will handle logic flow from user
#[derive(Debug,Clone)]
pub struct Handler {
    keys: HashMap <String, Action>,
}

// Generates a handler
pub fn init_handler() -> Handler {
    let mut ks: HashMap<String, Action> = HashMap::new();
    
    ks.insert(String::from("Quit")             ,Quit);
    ks.insert(String::from("MoveUp")           ,MoveUp);
    ks.insert(String::from("MoveDown")         ,MoveDown);
    ks.insert(String::from("MoveLeft")         ,MoveLeft);
    ks.insert(String::from("MoveRight")        ,MoveRight);
    ks.insert(String::from("EOL")              ,EOL);
    ks.insert(String::from("BOL")              ,BOL);
    ks.insert(String::from("SetActiveFilePath"),SetActiveFilePath);
    ks.insert(String::from("LoadFromFilePath") ,LoadFromFilePath);
    ks.insert(String::from("Save")             ,Save);
    ks.insert(String::from("PrintMini")        ,PrintMini);

    // Final returned value
    Handler {
	keys: ks,
    }
}

impl Handler {
    // Logic for user input in stdin
    pub fn handle_keypress(&self, frame: &mut Frame, mbuf: &mut MiniBuf, term: &Term) ->  Result<Action> {
	let raw_k = term.read_key()?;
	let k = parse_key(raw_k.clone(), mbuf, term);

	// Check if it's a known key
	match self.keys.contains_key(&k) {
	    true => Ok(self.keys[&k]),
	    false => self.unknown_keys(raw_k, frame, mbuf, term),
	}
    }

    // Handle keys that we know are not associated with actions
    fn unknown_keys(&self, raw_k: Key, frame: &mut Frame, mbuf: &mut MiniBuf, term: &Term) -> Result<Action> {
	let _success = match raw_k {
	    Key::Char('\u{4}') => frame.delete()?,         // Delete in place
	    Key::Backspace     => frame.backspace()?,	   // Delete backwards
	    Key::Enter         => frame.newline()?,        // Newline
	    Key::Char(c)       => frame.write_char(c)?,    // Write char to buffer
	    bad_k => {// Show the error text in the minibuffer and do nothing
		let err_text = format!("Not valid key press: {:?}", bad_k); 
		mbuf.show_err(err_text, term)?;
		true
	    },
	};

	Ok(DoNo)
    }
}

// Go from a console::Key to String
// See https://docs.rs/console/0.15.0/console/enum.Key.html
pub fn parse_key(raw_k: Key, mbuf: &mut MiniBuf, term: &Term) -> String {
    match raw_k {
	Key::Char('\u{11}') => { // C-q
	    String::from("Quit")
	},
	Key::Char('\u{10}') => { // C-p	    
	    String::from("MoveUp")
	},
	Key::Char('\u{e}') => { // C-n
	    String::from("MoveDown")
	}
	Key::Char('\u{2}') => { // C-b	    
	    String::from("MoveLeft")
	}
	Key::Char('\u{6}') => { // C-f
	    String::from("MoveRight")
	}
	Key::End => {  // C-e
	    String::from("EOL")
	}
	Key::Home => { // C-a
	    String::from("BOL")
	}
	Key::Char('\u{c}') => { // C-l
	    String::from("SetActiveFilePath")
	}
	Key::Char('\u{12}') => { // C-r
	    String::from("LoadFromFilePath")
	}
	Key::Char('\u{13}') => { // C-s
	    String::from("Save")
	},
	Key::Char('\0') => { // ` (i make this char with Shift-~)
	    String::from("PrintMini")
	}
	k => { // Anything else
	    let stxt = format!("{:?}", k);
	    let _mbuf_res = mbuf.show_success(stxt.clone(), term);
	    stxt
	},
    }
}



