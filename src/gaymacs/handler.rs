use std::io::Result;
use std::collections::HashMap;
use console::{Term, Key};

use crate::{Action, Action::*};
use crate::gaymacs::mini::MiniBuf;

// The Handler will handle logic flow from user
#[derive(Debug,Clone)]
pub struct Handler {
    keys: HashMap<String, Action>,
}

// Generates a handler
pub fn init_handler() -> Handler {
    let mut ks: HashMap<String, Action> = HashMap::new();
    
    ks.insert(String::from("InsertMode")       ,InsertMode);
    ks.insert(String::from("ClearBuf")         ,ClearBuf);
    ks.insert(String::from("Quit")             ,Quit);
    ks.insert(String::from("MoveUp")           ,MoveUp);
    ks.insert(String::from("MoveDown")         ,MoveDown);
    ks.insert(String::from("MoveLeft")         ,MoveLeft);
    ks.insert(String::from("MoveRight")        ,MoveRight);
    ks.insert(String::from("SetActiveFilePath"),SetActiveFilePath);
    ks.insert(String::from("Save")             ,Save);
    ks.insert(String::from("PrintMini")        ,PrintMini);

    // Final returned value
    Handler {
	keys: ks,
    }
}

impl Handler {
    // Logic for user input in stdin
    pub fn handle_keypress(&self, mbuf: &mut MiniBuf, term: &Term) ->  Result<Action> {
	let raw_k = term.read_key()?;
	let k = parse_key(raw_k);

	// Make sure its a valid key
	if self.keys.contains_key(&k) {
	    Ok(self.keys[&k])              // If valid, return associated action
	} else {
	    let err_text = format!("DEBUG: Not valid key press: {:?}", k); 
	    mbuf.show_err(err_text, term);
	    Ok(DoNo)                       // Do nothing
	}
    }
}

// Go from a console::Key to String
// See https://docs.rs/console/0.15.0/console/enum.Key.html
pub fn parse_key(raw_k: Key) -> String {
    match raw_k {
	Key::Escape => {
	    String::from("ClearBuf")
	}
	Key::Char('i') => {
	    String::from("InsertMode")
	}
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
	Key::Char('\u{c}') => { // C-l
	    String::from("SetActiveFilePath")
	}
	Key::Char('\u{13}') => { // C-s
	    String::from("Save")
	},
	Key::Char('\0') => { // ` (i make this char with Shift-~)
	    String::from("PrintMini")
	}
	k => { // Anything else
	    println!("DEBUG: Read key {:?}",k);
	    String::from("DoNo")
	},
    }
}



