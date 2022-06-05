use std::io::Result;
use crate::{Frame, Action, Action::*};
use crate::gaymacs::mini::MiniBuf;

// The entire window to be displayed in the terminal
#[derive(Debug)]
pub struct Window {
    frames: Vec<Frame>, // List of all frames
    aframe: Frame,      // Current Active frame
    mbuf:   MiniBuf,    // Minibuffer
}

// Create a new default window
pub fn init_win(def_frame: Frame) -> Window {
    let mut frames: Vec<Frame> = Vec::new();
    frames.push(def_frame.clone());
    return Window {
	frames: frames,
	aframe: def_frame,
	mbuf:   MiniBuf::from("minibuffer!"),
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
    pub fn ls_frames(&mut self, term: &console::Term) -> Result<()> {
	for frame in &self.frames {
	    let out = frame.name();
	    term.write_line(&out)?;
	};
	Ok(())
    }

    // Return the current active frame
    pub fn get_aframe(&mut self) -> Frame {
	return self.aframe.clone()
    }

    // Display the contents of the minibuffer
    pub fn popup_mini(&mut self, term: &console::Term) -> Result<()> {
	&self.mbuf.print(term)?;
	Ok(())
    }

    // Execute the commands that were passed by the user
    pub fn execute(&mut self, act: Action, term: &console::Term) -> Result<bool> {
	let mut out = true;
	match act {
	    Quit => {
		&term.write_line("");
		out = false;
	    },
	    Save => {
		out = self.aframe.save(&self.mbuf)?;
	    },
	    MoveUp => {
		term.move_cursor_up(1)?;
	    },
	    MoveDown => {
		term.move_cursor_down(1)?;
	    },
	    MoveLeft => {
		term.move_cursor_left(1)?;
	    },
	    MoveRight => {
		term.move_cursor_right(1)?;
	    },
	    PrintMini => {
		self.popup_mini(term)?;
	    },
	    _ => (),
	}

	Ok(out)
    }
}
