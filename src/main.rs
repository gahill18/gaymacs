/* Author: Garrett Hill */

use console::{Term, Key};
use std::io::Result;

// Use windows and frames in main functionality
mod gaymacs;
use crate::gaymacs::frame::{Frame, *};
use crate::gaymacs::window::{Window, *};
use crate::gaymacs::actions::{Action, Action::*};
use crate::gaymacs::handler::{handle_keypress, *};

// Setup the starting conditions for the editor
fn startup(term: &Term) -> Result<(Window, Frame)> {
    // Frame Name
    let fname: String = String::from("*startup*");
    // Frame's Initial Buffer text
    let fibuf: String = String::from("Splash!");
    // Starting frame with no file (scratch buff)
    let mut aframe: Frame = init_frame(0, fname, fibuf, None);
    // starting window
    let mut init_win: Window = init_win(aframe.clone());
    
    // List the frames
    init_win.ls_frames(&term)?;
    // Show the text of the first frame
    aframe.print(&term)?;
    // Show the minibuffer text
    init_win.popup_mini(&term)?;
    
    // Return the starting window and starting frame
    return Ok((init_win, aframe))
}



// Core functionality
fn main() -> Result<()> {
    // Terminal abstraction for easier management of terminal interaction
    let term = Term::stdout();
    term.clear_screen()?;
    term.show_cursor()?;

    // Starting window and starting frame
    let (mut win, mut aframe): (Window, Frame) = startup(&term)?;

    let mut clean = true;                       // We haven't interrupted yet
    let mut act = DoNo;                         // Default action is to do nothing
    while clean {
	act = handle_keypress(&term)?;	        // Get action from user input
	clean = win.execute(act, &term)?;	// Handle actions
    }

    // Exit successfully
    Ok(())
}
