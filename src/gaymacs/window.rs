use std::io::Result;
use num::clamp;
use console::Term;

use crate::*;
use crate::gaymacs::mini::MiniBuf;

// The entire window to be displayed in the terminal
pub struct Window {
    frames: Vec<Frame>, // List of all frames
    aframe: Frame,      // Current Active frame
    mbuf:   MiniBuf,    // Minibuffer
    mbup:   bool,       // Show minibuffer?
    term:   Term,       // Terminal to manage windows in
    handler: Handler,   // Handler for keypresses
}

// Create a new default window
pub fn init_win(def_frame: Frame, t: &Term, h: &Handler) -> Window {
    let fs: Vec<Frame> = vec![def_frame.clone()];

    Window {
	frames:  fs,
	aframe:  def_frame,
	mbuf:    MiniBuf::from("outs"),
	mbup:    false,
	term:    t.clone(),
	handler: h.clone(),
    }
}

impl Window {
    // Try to redraw to the terminal
    pub fn refresh(&self) -> Result<bool> {
	// Redraw
	self.term.clear_screen()?;
	self.aframe.print()?;
	if self.mbup {
	    self.mbuf.print(&self.term);
	}

	// Update cursor
	let (new_x,new_y) = fcur_to_tcur(self.aframe.cur(), &self.term);
	self.term.move_cursor_to(new_x, new_y)?;
	
	Ok(true)
    }
    
    // Add a frame to the window
    pub fn add_frame(&mut self, frame: Frame) -> Result<bool> {
	self.frames.push(frame.clone());   // Save the new frame
	self.aframe = frame;               // Switch active window to newest frame
	Ok(true)
    }

    // List the frames the window can show/switch to in the minibuffer
    pub fn ls_frames(&mut self) -> Result<bool> {
	for frame in &self.frames {       // For every frame in the list
	    let out = frame.name();       // Get the name of the frame
	    self.mbuf.show_success(out, &self.term)?;  // Print in the minibuffer
	};
	Ok(true)
    }

    // Alternate if the mini should be shown or not
    pub fn popup_mini(&mut self) -> Result<bool> {
	self.mbup = !self.mbup;
	Ok(true)
    }

    // Try to get new filepath from the user for buffer writing
    // The file doesn't have to exist, but the directory its in does
    pub fn get_path_from_user(&mut self) -> Result<bool> {
	let _ = &self.term.write_line("Desired filepath:");
	let path = self.term.read_line()?;
	self.aframe.set_path(path, &mut self.mbuf)?;
	Ok(true)
    }

    // Try to handle the next keypress from the user
    pub fn handle_keypress(&mut self) -> Result<Action> {
	self.handler.handle_keypress(&mut self.aframe, &mut self.mbuf, &self.term)
    }

    // Execute the commands that were passed by the user
    pub fn execute(&mut self, act: Action) -> Result<bool> {
	let (tr,tc): (u16,u16) = self.term.size();
	let (r,c): (usize, usize) = (tr.into(), tc.into());
	let l: usize = self.aframe.text().len();
	
	match act {
	    Quit => {		// Move to a newline before exiting
		self.term.write_line("")?; 
		Ok(false)
	    },
	    DoNo      => Ok(true),         // Do Nothing
	    Save      => self.aframe.save(&mut self.mbuf), // Save the current frame
	    MoveUp    => {		
		let old_i: usize = self.aframe.cur();
		// if there is room to move up, sub term's columns from old frame cur
		if old_i > c {
		    let new_cur = old_i - c;
		    self.aframe.set_cur(new_cur);
		}
		// No room to move up, so go to the beginning of the buffer
		else {
		    self.aframe.set_cur(0);
		}
		Ok(true)
	    },
	    MoveDown  => {
		let old_i: usize = self.aframe.cur();
		// If there is room to move down, add term's columns from old frame cur
		let new_cur = clamp(old_i + c, 0, l);
		self.aframe.set_cur(new_cur);
		Ok(true)
	    },
	    MoveLeft  => self.aframe.move_bck(),
	    MoveRight => self.aframe.move_fwd(),
	    EOL => {
		// Do some math to move to the end of the current line
		let old_i: usize = self.aframe.cur();
		let rem = old_i % c;
		let add = c - rem;
		self.aframe.set_cur(clamp(old_i + add, 0, l));
		Ok(true)
	    },
	    BOL => {
		// Do some math to move to the beginning of the current line
		let old_i: usize = self.aframe.cur();
		let sub = c;
		self.aframe.set_cur(old_i - clamp(sub, 0, old_i));
		Ok(true)
	    },
	    PrintMini => self.popup_mini(),
	    LoadFromFilePath  => self.aframe.load_from_path(&mut self.mbuf),
	    SetActiveFilePath => self.get_path_from_user(),
	    // Don't crash, just tell what went wrong		
	    c => {  
		let error_text = format!("failed to execute command {:?}", c);
		self.mbuf.show_err(error_text, &self.term)?;
		self.popup_mini();
		Ok(true)
	    }
	}
    }
}

// Convert the frame's buffer index to the term cursor's x/y coordinates
fn fcur_to_tcur(i: usize, term: &Term) -> (usize,usize) {
    let (_tr,tc) = term.size(); // rows and columns
    let x = i % (tc) as usize; // what column are we in?
    let y = i / (tc) as usize; // what row are we in?
    (x,y)
}
