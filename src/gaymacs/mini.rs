use std::io::*;

#[derive(Debug)]
pub struct MiniBuf {
    outs: Vec<String>,            // Text contents of minibuffer
    errs: Vec<Error>,      // List of past errors
}

impl MiniBuf {
    // Generate a minibuffer from a string
    pub fn from(s: &str) -> MiniBuf {
	let mut bs: Vec<String> = Vec::new();
	bs.push(String::from(s));

	let mut es: Vec<Error> = Vec::new();
	
	MiniBuf {
	    outs: bs,
	    errs: es,
	}
    }

    // Write the current contents of the minibuffer to the terminal
    pub fn print(&self, term: &console::Term) -> Result<()> {
	term.write_line(&format!("{:?}", &self.outs))?;
	term.write_line(&format!("{:?}", &self.errs))?;	
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
	self.outs.push(s);
	Ok(true)
    }
}
