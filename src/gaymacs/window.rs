use std::io::Result;
use console::Term;

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
	mbuf:   MiniBuf::from("minibuffer!"),
	term:   t.clone(),
    }
}

impl Window {
    // Add a frame to the window
    pub fn add_frame(&mut self, frame: Frame) -> () {
	&self.frames.push(frame.clone());
	// Switch active window to newest frame
	self.aframe = frame
    }

    // List the frames the window can show/switch to
    pub fn ls_frames(&mut self) -> Result<()> {
	for frame in &self.frames {
	    let out = frame.name();
	    self.term.write_line(&out)?;
	};
	Ok(())
    }

    // Return the current active frame
    pub fn get_aframe(&mut self) -> Frame {
	self.aframe.clone()
    }

    // Display the contents of the minibuffer
    pub fn popup_mini(&mut self) -> Result<()> {
	&self.mbuf.print(&self.term)?;
	Ok(())
    }

    // Ask the user for a file location
    // The file doesn't have to exist, but the directory its in does
    pub fn get_path_from_user(&mut self) -> Result<bool> {
	let path = self.term.read_line()?;
	self.aframe.set_path(path);
	Ok(true)
    }

    // Execute the commands that were passed by the user
    pub fn execute(&mut self, act: Action) -> Result<bool> {
	let mut out = true;
	match act {
	    Quit => {
		// Move to a new line then exit
		self.term.write_line("")?;
		out = false
	    },
	    Save => out = self.aframe.save(&mut self.mbuf)?,
	    MoveUp => self.term.move_cursor_up(1)?,
	    MoveDown => self.term.move_cursor_down(1)?,
	    MoveLeft => self.term.move_cursor_left(1)?,
	    MoveRight => self.term.move_cursor_right(1)?,
	    PrintMini => self.popup_mini()?,
	    SetActiveFilePath => out = self.get_path_from_user()?,
	    _ => out = false,
	};

	Ok(out)
    }
}
