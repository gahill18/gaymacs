use std::io::{Result, Error};
use console::{Term, Key};

use crate::{Frame, Action, Action::*};
use crate::gaymacs::mini::MiniBuf;

// The entire window to be displayed in the terminal
#[derive(Debug)]
pub struct Window {
    frames: Vec<Frame>, // List of all frames
    aframe: Frame,      // Current Active frame
    mbuf:   MiniBuf,    // Minibuffer
    term:   Term        // Terminal to manage windows in
}

// Create a new default window
pub fn init_win(def_frame: Frame, t: &Term) -> Window {
    let mut frames: Vec<Frame> = Vec::new();
    frames.push(def_frame.clone());
    return Window {
	frames: frames,
	aframe: def_frame,
	mbuf:   MiniBuf::from("outs"),
	term:   t.clone(),
    }
}

impl Window {
    pub fn refresh(&self) -> () {
	self.term.clear_screen();
	self.aframe.print();
    }
    
    // Add a frame to the window
    pub fn add_frame(&mut self, frame: Frame) -> () {
	&self.frames.push(frame.clone());
	// Switch active window to newest frame
	self.aframe = frame
    }

    // List the frames the window can show/switch to
    pub fn ls_frames(&mut self) -> Result<()> {
	for frame in &self.frames {       // For every frame in the list
	    let out = frame.name();       // 
	    self.term.write_line(&out)?;  //
	};
	Ok(())
    }

    // Return the current active frame
    pub fn get_aframe(&mut self) -> &Frame {
	&self.aframe
    }

    // Display the contents of the minibuffer
    pub fn popup_mini(&mut self) -> Result<bool> {
	self.mbuf.print(&self.term)?;
	Ok(true)
    }

    // Ask the user for a file location
    // The file doesn't have to exist, but the directory its in does
    pub fn get_path_from_user(&mut self) -> Result<bool> {
	let path = self.term.read_line()?;
	self.aframe.set_path(path, &mut self.mbuf)?;
	Ok(true)
    }

    pub fn insert_mode(&mut self) -> Result<bool> {
	let mut intrpt = false;

	// As long as no keyboard interrupts we can insert
	while !intrpt {
	    match self.term.read_key()? {
		// Exit insert mode
		Key::Escape => {
		    intrpt = true;
		},
		Key::Backspace => {
		    self.aframe.backspace();
		}
		// Add the character to the buffer
		c => {
		    self.aframe.insert(c)?;
		},
	    };
	}
	
	Ok(true)
    }

    // Execute the commands that were passed by the user
    pub fn execute(&mut self, act: Action) -> Result<bool> {
	match act {
	    Quit => {
		// Move to a new line then exit
		self.term.write_line("")?;
		Ok(false)
	    },
	    Save => {
		self.aframe.save(&mut self.mbuf)
	    },
	    ClearBuf => {
		self.aframe.clear_buf()
	    }
	    InsertMode => {
		self.insert_mode()
	    }
	    MoveUp => {
		self.term.move_cursor_up(1)?;
		Ok(true)
	    },
	    MoveDown => {
		self.term.move_cursor_down(1)?;
		Ok(true)
	    },
	    MoveLeft => {
		self.term.move_cursor_left(1)?;
		Ok(true)
	    },
	    MoveRight => {
		self.term.move_cursor_right(1)?;
		Ok(true)
	    },
	    PrintMini => {
		self.popup_mini()
	    },
	    SetActiveFilePath => {
		self.get_path_from_user()
	    },
	    c => {
		// self.mbuf.show_err(Error {
		//     repr: "test"
		// })?;
		println!("failed to execute {:?}", c);
		Ok(false)
	    }
	}
    }
}
