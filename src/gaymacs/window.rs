use std::io::Result;
use num::clamp;
use console::Term;

use crate::gaymacs::frame::Frame;
use crate::gaymacs::mini::MiniBuf;
use crate::gaymacs::modeline::Modeline;
use crate::gaymacs::handler::Handler;
use crate::gaymacs::killring::Killring;
use crate::*;


// The entire window to be displayed in the terminal
pub struct Window<'a> {
    aframe:  Frame<'a>,          // Current Active frame
    frames:  Vec<Frame<'a>>,     // List of all frames    
    mbuf:    MiniBuf<'a>,        // Minibuffer
    mbup:    bool,               // Show minibuffer?
    term:    &'a Term,           // Terminal to manage windows in
    handler: Handler<'a>,        // Handler for keypresses
    mdln:    Modeline<'a>,       // Modeline for window information
    kilr:    Killring,           // Killring for cut/paste
}

// Create a new default window
pub fn init_win (t: &Term) -> Window {
    // Starting frame
    let fnam     = String::from("Startup Buffer Name");
    let fbuf     = String::from("Startup Buffer Text");
    let fpth     = None; // No file path to start, scratch buffer	
    let af       = Frame::init_frame(fnam,fbuf,fpth,t);

    // List of frames
    let fs:   Vec<Frame> = vec![];

    // Killring
    let kr:   Killring = Killring::init_killring();

    // Final output
    let mut out = Window {	    
	aframe:  af,
	frames:  fs,
	mbuf:    MiniBuf::init_minibuf(&t),
	mdln:    Modeline::init_modeline(&t),
	handler: Handler::init_handler(&t),
	mbup:    false,
	term:    t,
	kilr:    kr,
    };

    // return the window
    out
}

impl <'a> Window<'a> {
    // Try to redraw to the terminal
    pub fn refresh(&self) -> Result<bool> {
	// Redraw
	self.term.clear_screen()?;
	self.aframe.print()?;
	if self.mbup { self.mbuf.print()?; }
	self.mdln.print()?;

	// Update cursor
	let (new_x,new_y) = fcur_to_tcur(self.aframe.cur(), &self.term);
	self.term.move_cursor_to(new_x, new_y)?;
	
	Ok(true)
    }
    
    // Add a frame to the window
    pub fn add_frame(&mut self, frame: Frame<'a>) -> Result<()> {
	self.aframe = frame;            // Switch active window to newest frame
	self.frames.push(self.aframe.clone());   // Save the new frame	
	Ok(())
    }

    // List the frames the window can show/switch to in the minibuffer
    pub fn ls_frames(&mut self) -> Result<bool> {
	for frame in &self.frames {        // For every frame in the list
	    let out = frame.name();        // Get the name of the frame
	    self.mbuf.show_success(out)?;  // Print in the minibuffer
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
	self.aframe.set_path(path, &mut self.mbuf)
    }

    // Try to handle the next keypress from the user
    pub fn handle_keypress(&mut self) -> Result<Action> {
	self.handler.handle_keypress(&mut self.aframe, &mut self.mbuf)
    }

    // Execute the commands that were passed by the user
    pub fn execute(&mut self, act: Action) -> Result<bool> {
	let (tr,tc): (u16,u16) = self.term.size();
	let (_r,c): (usize, usize) = (tr.into(), tc.into());
	let l: usize = self.aframe.text().len();
	
	match act {
	    Quit      => Ok(false),                        // Exit GAYMACS
	    DoNo      => Ok(true),                         // Do Nothing
	    Save      => self.aframe.save(&mut self.mbuf), // Save current frame
	    MoveUp    => {		
		let old_i: usize = self.aframe.cur();
		// if room to move up, sub term's columns from old frame cur
		if old_i > c {
		    let new_cur = old_i - c;
		    self.aframe.set_cur(new_cur);
		}
		// No room to move up, so go to the beginning of the buffer
		else { self.aframe.set_cur(0); }		
		Ok(true)
	    },
	    MoveDown  => {
		let old_i: usize = self.aframe.cur();
		// If room to move down, add term's cols to old frame cur
		let new_cur = clamp(old_i + c, 0, l);
		self.aframe.set_cur(new_cur);
		Ok(true)
	    },
	    MoveLeft  => self.aframe.move_bck(),
	    MoveRight => self.aframe.move_fwd(),
	    Eol => {
		// Do some math to move to the end of the current line
		let old_i: usize = self.aframe.cur();
		let add:   usize = c - (old_i % c);
		self.aframe.set_cur(clamp(old_i + add - 1, 0, l));
		Ok(true)
	    },
	    Bol => {
		// Do some math to move to the beginning of the current line
		let old_i: usize = self.aframe.cur();
		let rem = old_i % c;
		self.aframe.set_cur(old_i - rem);
		Ok(true)
	    },
	    Kill => self.aframe.kill(&mut self.kilr),
	    Yank => self.aframe.yank(&mut self.kilr),
	    PrintMini => self.popup_mini(),
	    LoadFromFilePath  => self.aframe.load_from_path(&mut self.mbuf),
	    SetActiveFilePath => self.get_path_from_user(),
	    // Don't crash, just tell what went wrong		
	    c => {  
		let error_text = format!("failed to execute command {:?}", c);
		self.mbuf.show_err(error_text)?;
		self.popup_mini()
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
