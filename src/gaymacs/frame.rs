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

    // Set the path variable so save works correctly
    pub fn set_path(&mut self, p: String, mbuf: &mut MiniBuf) -> Result<bool> {
	self.path = Some(p);
	let s_text = format!("path set as {:?}", self.path);
	mbuf.show_success(s_text, &self.term)?;
	Ok(true)
    }

    // Print to the terminal
    pub fn print(&self) -> Result<bool> {
	let _ = &self.term.write_line(&self.text());
	Ok(true)
    }

    // Clear the current buffer
    pub fn clear_buf(&mut self) -> Result<bool> {
	self.buf = String::from("");
	let _ = &self.term.clear_screen()?;
	Ok(true)
    }

    pub fn backspace(&mut self) -> Result<bool> {
	let _c = self.buf.pop();
	println!("DEBUG: backspace");
	Ok(true)
    }

    pub fn insert(&mut self, s: char) -> Result<bool> {
	self.buf = format!("{}{}", &self.buf, s);
	Ok(true)
    }

    // Write the buffer to the saved filepath
    pub fn save(&mut self, mbuf: &mut MiniBuf) -> Result<bool> {
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
		match File::create(&p) {
		    // If we opened the file, try to write the buffer contents
		    Ok(mut file) => match file.write_all(&self.buf.as_bytes()) {
			Ok(s) => {
			    //Show success
			    let s_text = format!("saved in {:?}",p);
			    mbuf.show_success(s_text, &self.term)
			},
			Err(s) =>
			    //Show error
			    mbuf.show_err(s.to_string(), &self.term),
		    },
		    // If we failed to open the file, show the error in mini
		    Err(s) => mbuf.show_err(s.to_string(), &self.term),
		}
	    },
	}
    }

    // Save the buffer in a new location obtained from the user
    pub fn save_as(&mut self, mbuf: &mut MiniBuf) -> Result<bool> {
	self.term.write_line("Save as: ")?;
	self.path = Some(self.term.read_line()?);
	self.save(mbuf)
    }
}
