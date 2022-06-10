use std::io::Result;
use num::clamp;
use console::Term;

use crate::*;
use crate::gaymacs::mini::MiniBuf;

// The entire window to be displayed in the terminal
#[derive(Debug)]
pub struct Window {
    frames: Vec<Frame>, // List of all frames
    aframe: Frame,      // Current Active frame
    mbuf:   MiniBuf,    // Minibuffer
    term:   Term,       // Terminal to manage windows in
    handler: Handler,   // Handler for keypresses
}

// Create a new default window
pub fn init_win(def_frame: Frame, t: &Term, h: &Handler) -> Window {
    let mut fs: Vec<Frame> = vec![def_frame.clone()];

    Window {
	frames:  fs,
	aframe:  def_frame,
	mbuf:    MiniBuf::from("outs"),
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

    // List the frames the window can show/switch to
    pub fn ls_frames(&mut self) -> Result<bool> {
	for frame in &self.frames {       // For every frame in the list
	    let out = frame.name();       // Get the name of the frame
	    self.mbuf.show_success(out, &self.term)?;  // Print in the minibuffer
	};
	Ok(true)
    }

    // Borrow the current active frame
    pub fn aframe(&mut self) -> &Frame {
	&self.aframe
    }

    // Borrow the minibuffer
    pub fn mini(&mut self) -> &MiniBuf {
	&self.mbuf
    }

    // Try to display the contents of the minibuffer
    pub fn popup_mini(&mut self) -> Result<bool> {
	self.mbuf.print(&self.term)?;
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
	match act {
	    Quit => {		
		self.term.write_line("")?; // Move to a newline before exiting
		Ok(false)
	    },
	    DoNo      => Ok(true),         // Do Nothing
	    Save      => self.aframe.save(&mut self.mbuf), // Save the current frame
	    MoveUp    => {
		let (tr,tc): (u16,u16) = self.term.size();
		let (r,c): (usize, usize) = (tr.into(), tc.into());
		let old_i: usize = self.aframe.cur();
		// if there is room to move up, sub term's columns from old frame cur
		if old_i > c {
		    let new_cur = old_i - c;
		    self.aframe.set_cur(new_cur);
		} else {
		    self.aframe.set_cur(0);
		}
		Ok(true)
	    },
	    MoveDown  => {
		let l = self.aframe.text().len();
		let (tr,tc): (u16,u16) = self.term.size();
		let (r,c): (usize, usize) = (tr.into(), tc.into());
		let old_i: usize = self.aframe.cur();
		// If there is room to move down, add term's columns from old frame cur
		let new_cur = clamp(old_i + c, 0, l);
		self.aframe.set_cur(new_cur);
		Ok(true)
	    },
	    MoveLeft  => self.aframe.move_bck(&mut self.mbuf),
	    MoveRight => self.aframe.move_fwd(&mut self.mbuf),
	    PrintMini => self.popup_mini(),
	    LoadFromFilePath  => self.aframe.load_from_path(&mut self.mbuf),
	    SetActiveFilePath => self.get_path_from_user(),
	    // Don't crash, just tell what went wrong		
	    c => {  
		let error_text = format!("failed to execute command {:?}", c);
		self.mbuf.show_err(error_text, &self.term)?;		
		Ok(true)
	    }
	}
    }
}

// Convert the frame's buffer index to the term cursor's x/y coordinates
fn fcur_to_tcur(i: usize, term: &Term) -> (usize,usize) {
    let (tr,tc) = term.size(); // rows and columns
    let x = i % (tc) as usize; // what column are we in?
    let y = i / (tc) as usize; // what row are we in?
    (x,y)
}
