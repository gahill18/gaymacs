use std::io::Result;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use num::clamp;
use console::Term;

use crate::gaymacs::{mini::MiniBuf, killring::Killring};

// The individual frames that can be displayed in the window
#[derive(Debug,Clone)]
pub struct Frame <'a> {
    name: String,              // name of frame to display in mini/modeline
    buf:  String,              // buffer contents
    cur:  usize,               // cursor location in the buffer
    path: Option<String>,      // if saved document, store its location
    term: &'a Term,               // console this frame is assigned to by win
}

// Where you actually edit the text
impl <'a> Frame<'a> {
    // Takes uniq id, frame name, and starting buf text
    // returns new frame
    pub fn init_frame(n: String, b: String, p: Option<String>, t: &'a Term) -> Frame<'a> {
	Frame {
	    name: n,
	    buf:  b,
	    cur:  0,
	    path: p,
	    term: t,
	}
    }

    // Print to the terminal
    pub fn print(&self) -> Result<()> {
	self.term.write_line(&self.text())
    }
    
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
	self.cur
    }

    // Update the cursor
    pub fn set_cur(&mut self, i: usize) -> Result<()> {
	Ok(self.cur = i)
    }

    // Set the path variable so saving works correctly
    pub fn set_path(&mut self, p: String, mbuf: &mut MiniBuf) -> Result<()> {
	self.path = Some(p);
	let s_text = format!("path set as {:?}", self.path);
	mbuf.show_success(s_text)
    }

    // Load a file into buffer
    pub fn load_from_path(&mut self, mbuf: &mut MiniBuf) -> Result<()> {
	match &self.path {	                        // Ensure valid path
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
				mbuf.show_success(fcontents)
			    },
			    // We didn't read it :(
			    Err(s) => mbuf.show_err(s.to_string()),
			}
		    },
		    // Didn't open it :(
		    Err(s) => mbuf.show_err(s.to_string()),
		}
	    },	    
	    None   => {                                 // No path :(
		let err_text = format!("no filepath for buffer {:?}", self.name);
		mbuf.show_err(err_text)
	    },
	}
    }

    // Delete the character behind the cursor
    pub fn backspace(&mut self) -> Result<()> {
	if self.cur > 0 {
	    let i = self.cur;
	    let _c = self.buf.remove(i-1);

	    // Only update if we arent at the beginning of the string
	    if i > 0 {
		self.cur = i - 1;
	    }
	}
	Ok(())
    }

    // Delete the character under the cursor
    pub fn delete(&mut self) -> Result<()> {
	let l = self.text().len();
	if l > 0 && self.cur < l {
	    let i = self.cur;
	    let _c = self.buf.remove(i);
	}		
	Ok(())
    }

    // Add the next character to the buffer
    pub fn write_char(&mut self, c: char) -> Result<()> {	
	self.buf.insert(self.cur, c);
	self.cur += 1;
	Ok(())
    }

    // Add a line break to the buffer
    pub fn newline(&mut self) -> Result<()> {
	// TODO: Fix cursor placement
	self.write_char('\n')
    }

    // Write the buffer to the saved filepath
    pub fn save(&mut self, mbuf: &mut MiniBuf) -> Result<()> {
	match &self.path {
	    // If no file has been initialized, do so now
	    None => self.save_as(mbuf),
	    // If a file exists, overwrite it's contents with current buffer's
	    Some(path) => {
		// Format the path text so we can read it
		let p = Path::new(path);
		// Attempt to open file 
		match File::create(&p) {
		    // On file open success, try to write the buffer contents
		    Ok(mut file) => match file.write_all(self.buf.as_bytes()) {
			//Show file write success
			Ok(_) => {			    
			    let s_text = format!("saved in {:?}",p);
			    mbuf.show_success(s_text)
			},
			//Show file write error
			Err(s) => mbuf.show_err(s.to_string()),
		    },
		    // Show file open error
		    Err(s) => mbuf.show_err(s.to_string()),
		}
	    },
	}
    }

    // Save the buffer in a new location obtained from the user
    pub fn save_as(&mut self, mbuf: &mut MiniBuf) -> Result<()> {
	self.term.write_line("Save as: ")?;
	self.path = Some(self.term.read_line()?);
	self.save(mbuf)
    }

    // Try to move the buffer index one step closer to the end of the buffer
    pub fn move_fwd(&mut self) -> Result<()> {
	let l = self.buf.len();
	let new_i = clamp(self.cur() + 1, 0, l);
	self.set_cur(new_i)
    }

    // Try to move the buffer index one step closer to the start of the buffer
    pub fn move_bck(&mut self) -> Result<()> {
	let l = self.buf.len();
	let new_i = clamp(self.cur(), 1, l) - 1;
	self.set_cur(new_i)
    }

    // Store the rest of the current line in the killring
    pub fn kill(&mut self, kilr: &mut Killring) -> Result<()> {
	let (tr,tc): (u16,u16) = self.term.size();
	let (_r,c): (usize, usize) = (tr.into(), tc.into());
	let l    = self.text().len();
	let i    = self.cur;
	let rem  = c - (i % c);
	let eol  = clamp(i + rem, 0, l);
	let text = self.buf.drain(i .. eol).collect();	
	kilr.kill(text)
    }

    // Write the most recent kill from the killring to the buffer
    pub fn yank(&mut self, kilr: &mut Killring) -> Result<()> {
	let text = kilr.yank()?;
	Ok(self.buf.insert_str(self.cur, &text))
    }
}
