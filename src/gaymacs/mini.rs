use std::io::*;

#[derive(Debug)]
pub struct MiniBuf {
    buf: String,     // Text contents of minibuffer
    errs: Vec<Error>, // List of past errors
}

impl MiniBuf {
    // Generate a minibuffer from a string
    pub fn from(s: &str) -> MiniBuf {
	return MiniBuf {
	    buf: String::from(s),
	    errs: Vec::new(),
	}
    }

    // Write the current contents of the minibuffer to the terminal
    pub fn print(&self, term: &console::Term) -> Result<()> {
	term.write_line(&self.buf)?;
	Ok(())
    }

    // Update the contents of the minibuffer to show the current error
    pub fn show_err(&mut self, e: Error) -> Result<bool> {
	self.errs.push(e);
	Ok(true)
    }

    // Update the contents of the minibuffer to show the output of a successfull
    // execution
    pub fn show_success(&mut self, s: String) -> Result<bool> {
	self.buf = s;
	Ok(true)
    }
}
