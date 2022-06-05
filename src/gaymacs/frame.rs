use std::io::Result;
use std::fs::File;
use std::path::Path;

use crate::gaymacs::mini::MiniBuf;

// The individual frames that can be displayed in the window
#[derive(Debug,Clone)]
pub struct Frame {
    id: u16,                   // unique identifier and ordering
    name: String,              // name of the frame that will display in mini
    buf: String,               // buffer contents (the text we're editing)
    path: Option<String>,      // If this is a saved document, store its location
}


// Takes a unique id, frame name, and starting buffer text, and returns new frame
pub fn init_frame(uid: u16, n: String, b: String, p: Option<String>) -> Frame {
    return Frame {
	id: uid,
	name: n,
	buf: b,
	path: p,
    }
}

// Where you actually edit the text
impl Frame {
    // Getter for name
    pub fn name(&self) -> String {
	return self.name.clone()
    }

    // Getter for text
    pub fn text(&self) -> String {
	return self.buf.clone()
    }

    // Print to the terminal
    pub fn print(&self, term: &console::Term) -> Result<()> {
	term.write_line(&self.text())?;
	Ok(())
    }

    // Write the buffer to the saved filepath
    pub fn save(&self, mbuf: &MiniBuf) -> Result<bool> {
	match &self.path {
	    // If no file has been initialized, do so now
	    None => {	    
		return self.save_as(mbuf)
	    },
	    // If a file exists, overwrite it's contents with current buffer's
	    Some(path) => {   
		Ok(true)
	    },
	}
    }

    pub fn save_as(&self, mbuf: &MiniBuf) -> Result<bool> {
	return Ok(true)
    }
}
