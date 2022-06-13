use std::io::*;
use console::Term;

#[derive(Debug,Clone)]
pub struct MiniBuf<'a> {
    outs: Vec<String>,            // Text contents of minibuffer
    errs: Vec<String>,            // List of past errors
    term: &'a Term,               // Terminal to print to
}

impl <'a> MiniBuf<'a> {
    // Generate a minibuffer from a string
    pub fn init_minibuf(t: &'a Term) -> MiniBuf<'a> {
	let bs: Vec<String> = vec![String::from("Success List")];
	let es: Vec<String> = vec![String::from("Error List")];

	// Return established minibuf
	MiniBuf {
	    outs: bs,
	    errs: es,
	    term: t,
	}
    }

    // Write the current contents of the minibuffer to the terminal
    pub fn print(&self) -> Result<()> {
	self.term.write_line(&format!("\n{:?}\n{:?}", &self.outs,&self.errs))
    }

    // Update the contents of the minibuffer to show the current error
    pub fn show_err(&mut self, e: String) -> Result<()> {
	self.term.write_line(&e)?;
	Ok(self.errs.push(e))
    }

    // Try to show the output of a successfull execution
    pub fn show_success(&mut self, s: String) -> Result<()> {
	self.term.write_line(&s)?;
	Ok(self.outs.push(s))
    }
}
