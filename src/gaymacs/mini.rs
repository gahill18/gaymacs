use std::io::*;
use console::Term;

use crate::gaymacs::killring::Killring;

#[derive(Debug,Clone)]
pub struct MiniBuf {
    outs: Vec<String>,            // Text contents of minibuffer
    errs: Vec<String>,            // List of past errors
    kilr: Killring,               // Minibuffers handle the killring
}

impl MiniBuf {
    // Generate a minibuffer from a string
    pub fn init_minibuf() -> MiniBuf {
	let bs: Vec<String> = vec![String::from("Success List")];
	let es: Vec<String> = vec![String::from("Error List")];
	let k:  Killring    = Killring::init_killring();

	// Return established minibuf
	MiniBuf {
	    outs: bs,
	    errs: es,
	    kilr: k,
	}
    }

    // Write the current contents of the minibuffer to the terminal
    pub fn print(&self, term: &Term) -> Result<()> {
	term.write_line(&format!("\n{:?}\n{:?}", &self.outs,&self.errs))?;
	Ok(())
    }

    // Update the contents of the minibuffer to show the current error
    pub fn show_err(&mut self, e: String, term: &Term) -> Result<bool> {
	self.errs.push(e.clone());
	term.write_line(&e)?;
	Ok(true)
    }

    // Try to show the output of a successfull execution
    pub fn show_success(&mut self, s: String, term: &Term) -> Result<bool> {
	self.outs.push(s.clone());
	term.write_line(&s)?;
	Ok(true)
    }
}
