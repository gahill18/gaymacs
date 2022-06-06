use std::io::*;
use console::Term;

#[derive(Debug)]
pub struct MiniBuf {
    outs: Vec<String>,            // Text contents of minibuffer
    errs: Vec<String>,      // List of past errors
}

impl MiniBuf {
    // Generate a minibuffer from a string
    pub fn from(s: &str) -> MiniBuf {
	let mut bs: Vec<String> = Vec::new();
	bs.push(String::from(s));

	let mut es: Vec<String> = Vec::new();
	
	MiniBuf {
	    outs: bs,
	    errs: es,
	}
    }

    // Write the current contents of the minibuffer to the terminal
    pub fn print(&self, term: &Term) -> Result<()> {
	term.write_line(&format!("{:?}", &self.outs))?;
	term.write_line(&format!("{:?}", &self.errs))?;	
	Ok(())
    }

    // Update the contents of the minibuffer to show the current error
    pub fn show_err(&mut self, e: String, term: &Term) -> Result<bool> {
	self.errs.push(e.clone());
	term.write_line(&e);
	Ok(true)
    }

    // Update the contents of the minibuffer to show the output of a successfull
    // execution
    pub fn show_success(&mut self, s: String, term: &Term) -> Result<bool> {
	self.outs.push(s.clone());
	term.write_line(&s);
	Ok(true)
    }
}
