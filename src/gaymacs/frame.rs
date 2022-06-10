use std::io::Result;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use num::clamp;
use console::Term;

use crate::gaymacs::mini::MiniBuf;

// The individual frames that can be displayed in the window
#[derive(Debug,Clone)]
pub struct Frame {
    id: u16,                   // unique identifier and ordering
    name: String,              // name of the frame that will display in mini
    buf: String,               // buffer contents (the text we're editing)
    cur: usize,                // Cursor location in the buffer
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
	cur: 0,
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

    // Getter for cursor
    pub fn cur(&self) -> usize {
	self.cur.clone()
    }

    // Getter for path
    pub fn path(&self) -> Option<String> {
	self.path.clone()
    }

    pub fn set_cur(&mut self, i: usize) -> () {
	self.cur = i;
    }

    // Set the path variable so saving works correctly
    pub fn set_path(&mut self, p: String, mbuf: &mut MiniBuf) -> Result<bool> {
	self.path = Some(p);
	let s_text = format!("path set as {:?}", self.path);
	mbuf.show_success(s_text, &self.term)?;
	Ok(true)
    }

    // Load a file into buffer
    pub fn load_from_path(&mut self, mbuf: &mut MiniBuf) -> Result<bool> {
	match &self.path {	        // Make sure we have a valid path
	    Some(p) => {	                        // Path is valid!
		let path = Path::new(&p);
		match &mut File::open(&path) {		// Try to open the path
		    Ok(file) => {		        // We opened it!
			let mut fcontents = String::new();
			// Try to read the file
			match file.read_to_string(&mut fcontents) {
			    // We read it!
			    Ok(_)  => {                 
				self.buf = fcontents.clone();
				mbuf.show_success(fcontents, &self.term)?
			    },
			    // We didn't read it :(
			    Err(s) => mbuf.show_err(s.to_string(), &self.term)?,
			};
		    },		    
		    Err(s)   => {                       // Didn't open it :(
			mbuf.show_err(s.to_string(), &self.term)?;
		    },
		}
	    },	    
	    None   => {                                 // No path :(
		let err_text = format!("no filepath for buffer {:?}", self.name);
		mbuf.show_err(err_text, &self.term)?;
	    },
	}
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

    // Delete the character behind the cursor
    pub fn backspace(&mut self) -> Result<bool> {
	if self.cur > 0 {
	    let i = self.cur;
	    let _c = self.buf.remove(i-1);

	    // Only update if we arent at the beginning of the string
	    if i > 0 {
		self.cur = i - 1;
	    }
	}
		
	Ok(true)
    }

    // Delete the character under the cursor
    pub fn delete(&mut self) -> Result<bool> {
	if self.buf.len() > 0 {
	    let i = self.cur;
	    let _c = self.buf.remove(i);
	}
		
	Ok(true)
    }

    // Add the next character to the buffer
    pub fn write_char(&mut self, c: char) -> Result<bool> {	
	self.buf.insert(self.cur, c);
	self.cur = self.cur + 1;
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
		    Ok(mut file) => match file.write_all(self.buf.as_bytes()) {
			Ok(s) => {
			    //Show file write success
			    let s_text = format!("saved in {:?}",p);
			    mbuf.show_success(s_text, &self.term)
			},
			Err(s) =>
			    //Show file write error
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

    // MOVEMENT FUNCTIONS

    pub fn move_fwd(&mut self, mbuf: &mut MiniBuf) -> Result<bool> {
	let l = self.buf.len();
	let new_i = clamp(self.cur() + 1, 0, l);
	self.set_cur(new_i);
	Ok(true)
    }

    pub fn move_bck(&mut self, mbuf: &mut MiniBuf) -> Result<bool> {
	let l = self.buf.len();
	let new_i = clamp(self.cur(), 1, l-1) - 1;
	self.set_cur(new_i);
	Ok(true)
    }
}
