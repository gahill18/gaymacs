use std::io::Result;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use console::Term;

use crate::gaymacs::mini::MiniBuf;

// The individual frames that can be displayed in the window
#[derive(Debug,Clone)]
pub struct Frame {
    id: u16,                   // unique identifier and ordering
    name: String,              // name of the frame that will display in mini
    buf: String,               // buffer contents (the text we're editing)
    path: Option<String>,      // If this is a saved document, store its location
    term: Term,                // The console this frame is assigned to by win
}


// Takes a unique id, frame name, and starting buffer text, and returns new frame
pub fn init_frame(uid: u16, n: String, b: String,
		  p: Option<String>, t: &Term) -> Frame {
    Frame {
	id: uid,
	name: n,
	buf: b,
	path: p,
	term: t.clone(),
    }
}

// Where you actually edit the text
impl Frame {
    // Getter for name
    pub fn name(&self) -> String {
	self.name.clone()
    }

    // Getter for text
    pub fn text(&self) -> String {
	self.buf.clone()
    }

    pub fn set_path(&mut self, p: String) -> Result<bool> {
	self.path = Some(p);
	Ok(true)
    }

    // Print to the terminal
    pub fn print(&self) -> Result<bool> {
	&self.term.write_line(&self.text())?;
	Ok(true)
    }

    // Write the buffer to the saved filepath
    pub fn save(&self, mbuf: &mut MiniBuf) -> Result<bool> {
	match &self.path {
	    // If no file has been initialized, do so now
	    None => {	    
		self.save_as(mbuf)
	    },
	    // If a file exists, overwrite it's contents with current buffer's
	    Some(path) => {
		// Format the path text so we can read it
		let p = Path::new(path);
		// Attempt to open file 
		let mut file = match File::create(&path) {
		    // If we opened the file, try to write the buffer contents
		    Ok(mut file) => match file.write_all(&self.buf.as_bytes()) {
			Ok(i) => {
			    println!("DEBUG: file saved");
			    Ok(true) // Success!
			},
			Err(s) => mbuf.show_err(s), // Show the error
		    },
		    // If we failed to open the file, show the error in mini
		    Err(s) => mbuf.show_err(s),
		};
		
		// If we made it this far, no problems
		Ok(true)
	    },
	}
    }

    // Save the buffer in a new location obtained from the user
    pub fn save_as(&self, mbuf: &MiniBuf) -> Result<bool> {
	println!("DEBUG: Save As Started");
	return Ok(true)
    }
}
