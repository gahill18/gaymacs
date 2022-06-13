use std::io::Result;
use std::collections::HashMap;
use console::{Term, Key};

use crate::gaymacs::{mini::MiniBuf, frame::Frame, actions::Action};
//use crate::{Action, gaymacs::mini::MiniBuf, Frame,};
use crate::{Action::*,};
// use crate::gaymacs::mini::MiniBuf;

// The Handler will handle logic flow from user
#[derive(Debug,Clone)]
pub struct Handler<'a> {
    keys: HashMap <String, Action>,
    term: &'a Term,
}

impl <'a> Handler<'a> {
    // Generates a handler
    pub fn init_handler(t: &'a Term) -> Handler<'a> {
	let mut ks: HashMap<String, Action> = HashMap::new();
	
	ks.insert(String::from("Quit")             ,Quit);
	ks.insert(String::from("MoveUp")           ,MoveUp);
	ks.insert(String::from("MoveDown")         ,MoveDown);
	ks.insert(String::from("MoveLeft")         ,MoveLeft);
	ks.insert(String::from("MoveRight")        ,MoveRight);
	ks.insert(String::from("Eol")              ,Eol);
	ks.insert(String::from("Bol")              ,Bol);
	ks.insert(String::from("Kill")             ,Kill);
	ks.insert(String::from("Yank")             ,Yank);
	ks.insert(String::from("SetActiveFilePath"),SetActiveFilePath);
	ks.insert(String::from("LoadFromFilePath") ,LoadFromFilePath);
	ks.insert(String::from("Save")             ,Save);
	ks.insert(String::from("PrintMini")        ,PrintMini);

	// Final returned value
	Handler {
	    keys: ks,
	    term: t,
	}
    }

    // Logic for user input in stdin
    pub fn handle_keypress(&self, frame: &'a mut Frame, mbuf: &'a mut MiniBuf) ->  Result<Action> {
	let raw_k = self.term.read_key()?;
	let k = parse_key(raw_k.clone(), mbuf);

	// Check if it's a known key
	match self.keys.contains_key(&k) {
	    true => Ok(self.keys[&k]),
	    false => self.unknown_keys(raw_k, frame, mbuf),
	}
    }

    // Handle keys that we know are not associated with actions
    pub fn unknown_keys(&self, raw_k: Key, frame: &mut Frame, mbuf: &mut MiniBuf) -> Result<Action> {
	let _success = match raw_k {
	    Key::Char('\u{4}') => frame.delete()?,         // Delete in place
	    Key::Backspace     => frame.backspace()?,	   // Delete backwards
	    Key::Enter         => frame.newline()?,        // Newline
	    // Write char to buffer if it isnt a control code
	    Key::Char(c)       => {
		if ! c.is_control() {
		    frame.write_char(c)?
		}
		else {
		    let err_text = format!("Not valid character: {:?}", c); 
		    mbuf.show_err(err_text)?;
		    true
		}
	    },
	    bad_k => {// Show the error text in the minibuffer and do nothing
		let err_text = format!("Not valid key press: {:?}", bad_k); 
		mbuf.show_err(err_text)?;
		true
	    },
	};

	Ok(DoNo)
    }
}

// Go from a console::Key to String
// See https://docs.rs/console/0.15.0/console/enum.Key.html
// See https://en.wikipedia.org/wiki/List_of_Unicode_characters#Control_codes
fn parse_key(raw_k: Key, mbuf: &mut MiniBuf) -> String {
    match raw_k {
	Key::Char('\u{11}') => String::from("Quit"), // C-q
	Key::Char('\u{10}') => String::from("MoveUp"), // C-p
	Key::Char('\u{e}')  => String::from("MoveDown"), // C-n
	Key::Char('\u{2}')  => String::from("MoveLeft"), // C-b
	Key::Char('\u{6}')  => String::from("MoveRight"), // C-f
	Key::End  => String::from("Eol"), // C-e
	Key::Home => String::from("Bol"), // C-a
	Key::Char('\u{b}')  => String::from("Kill"), // C-k
	Key::Char('\u{19}') => String::from("Yank"), // C-l
	Key::Char('\u{c}')  => String::from("SetActiveFilePath"), // C-l
	Key::Char('\u{12}') => String::from("LoadFromFilePath"), // C-r
	Key::Char('\u{13}') => String::from("Save"), // C-s
	Key::Char('\u{f}')  => String::from("PrintMini"), // C-o
	k => { // Anything else
	    let stxt = format!("{:?}", k);
	    let _mbuf_res = mbuf.show_success(stxt.clone());
	    stxt
	},
    }
}



