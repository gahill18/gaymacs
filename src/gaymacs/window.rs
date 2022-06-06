use std::io::Result;
use console::{Term, Key};

use crate::{*, Action::*};
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
    let mut fs: Vec<Frame> = Vec::new();
    fs.push(def_frame.clone());

    Window {
	frames:  fs,
	aframe:  def_frame,
	mbuf:    MiniBuf::from("outs"),
	term:    t.clone(),
	handler: h.clone(),
    }
}

impl Window {
    pub fn refresh(&self) -> Result<bool> {
	self.term.clear_screen()?;
	self.aframe.print()
    }
    
    // Add a frame to the window
    pub fn add_frame(&mut self, frame: Frame)  {
	self.frames.push(frame.clone());  // Save the new frame
	self.aframe = frame               // Switch active window to newest frame
    }

    // List the frames the window can show/switch to
    pub fn ls_frames(&mut self) -> Result<bool> {
	for frame in &self.frames {       // For every frame in the list
	    let out = frame.name();       // Get the name of the frame
	    self.mbuf.show_success(out, &self.term)?;  // Print in the minibuffer
	};
	Ok(true)
    }

    // Return the current active frame
    pub fn aframe(&mut self) -> &Frame {
	&self.aframe
    }

    pub fn mini(&mut self) -> &MiniBuf {
	&self.mbuf
    }

    // Display the contents of the minibuffer
    pub fn popup_mini(&mut self) -> Result<bool> {
	self.mbuf.print(&self.term)?;
	Ok(true)
    }

    // Ask the user for a file location
    // The file doesn't have to exist, but the directory its in does
    pub fn get_path_from_user(&mut self) -> Result<bool> {
	&self.term.write_line("Desired filepath:");
	let path = self.term.read_line()?;
	self.aframe.set_path(path, &mut self.mbuf)?;
	Ok(true)
    }

    // Handle text insertion/deletion
    pub fn insert_mode(&mut self) -> Result<bool> {
	let mut intrpt = false;
	// As long as no keyboard interrupts we can insert
	while !intrpt {
	    // Update the screen		
	    let _ = &self.refresh();
	    match self.term.read_key()? {
		Key::Escape => {		// Exit insert mode
		    intrpt = true;
		},
		Key::Backspace => {
		    self.aframe.backspace()?;
		}
		Key::Enter => {
		    self.aframe.insert('\n')?;
		}	
		// Add the character to the buffer
		Key::Char(c) => {
		    self.aframe.insert(c)?;
		}
		k => {
		    let error_text = format!("Didn't recognize key {:?}",k);
		    self.mbuf.show_err(error_text, &self.term)?;
		},
	    };
	}
	
	Ok(true)
    }

    // Execute the commands that were passed by the user
    pub fn execute(&mut self, act: Action) -> Result<bool> {
	let _ = &self.refresh();                          // Update the screen
	match act {
	    Quit => {
		// Move to a new line then exit
		self.term.write_line("")?;
		Ok(false)
	    },
	    Save => {
		// Save the current frame
		self.aframe.save(&mut self.mbuf)
	    },
	    ClearBuf => {
		// Wipe the current frame's buffer
		self.aframe.clear_buf()
	    }
	    InsertMode => {
		// Enter insert mode
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
	    DoNo => {
		// Don't do anything
		Ok(true)
	    },
	    c => {
		// Don't crash, just tell what went wrong
		let error_text = format!("failed to execute {:?}", c);
		self.mbuf.show_err(error_text, &self.term);
		Ok(true)
	    }
	}
    }
}
