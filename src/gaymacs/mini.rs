use std::io::Result;

#[derive(Debug)]
pub struct MiniBuf {
    buf: String,     // Text contents of minibuffer
}

impl MiniBuf {
    // Generate a minibuffer from a string
    pub fn from(s: &str) -> MiniBuf {
	return MiniBuf {
	    buf: String::from(s),
	}
    }

    // Write the current contents of the minibuffer to the terminal
    pub fn print(&self, term: &console::Term) -> Result<()> {
	term.write_line(&self.buf)?;
	Ok(())
    }
}
