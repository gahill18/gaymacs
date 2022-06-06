/* Author: Garrett Hill */

use console::Term;
use std::io::Result;

// Use windows and frames in main functionality
mod gaymacs;
use crate::gaymacs::frame::{Frame, *};
use crate::gaymacs::window::{Window, *};
use crate::gaymacs::actions::{Action, Action::*};
use crate::gaymacs::handler::*;

// Setup the starting conditions for the editor
fn startup(term: &Term) -> Result<(Window, Frame)> {
    let fname: String = String::from("*scratch*");  // Frame Name
    let fibuf: String = String::from("Splash!");    // Frame's Initial Buffer text

    // Starting frame with no file (scratch buf)
    let mut aframe: Frame = init_frame(0, fname, fibuf, None, term);
    // starting window
    let mut init_win: Window = init_win(aframe.clone(), term);
    
    aframe.print()?;           // Show the text of the first frame
//    init_win.ls_frames()?;     // List the frames
//    init_win.popup_mini()?;    // Show the minibuffer text
    
    
    Ok((init_win, aframe))     // Return the starting window and starting frame
}



// Core functionality
fn main() -> Result<()> {
    // Terminal abstraction for easier management of terminal interaction
    let term = Term::stdout();
    term.clear_screen()?;
    term.show_cursor()?;

    // Starting window and starting frame
    let (mut win, mut aframe): (Window, Frame) = startup(&term)?;
    // Get the event handler (default or user provided)
    let handler: Handler = init_handler();

    let mut clean = true;                       // We haven't interrupted yet
    let mut act = DoNo;                         // Default action is to do nothing
    while clean {                               // If no interrupts
	act = handler.handle_keypress(&term)?;	// Get action from user input
	clean = win.execute(act)?;	        // Handle actions
    }

    // TODO: Separate into navigation and edit modules

    // Exit successfully
    Ok(()) 
}
