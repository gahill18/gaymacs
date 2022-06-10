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
fn startup(term: &Term, handler: &Handler) -> Result<(Window, Frame)> {
    let fname: String = String::from("*scratch*");  // Frame Name
    let fibuf: String = String::from("Splash!");    // Frame's Initial Buffer text

    // Starting frame with no file (scratch buf)
    let mut aframe: Frame = init_frame(0, fname, fibuf, None, term);
    // starting window
    let mut init_win: Window = init_win(aframe.clone(), term, handler);
    
    aframe.print()?;           // Show the text of the first frame
    Ok((init_win, aframe))     // Return the starting window and starting frame
}



// Core functionality
fn main() -> Result<()> {
    // Terminal abstraction for easier management of terminal interaction
    let term = Term::stdout();
    term.clear_screen()?;
    term.show_cursor()?;

    // Get the event handler (default or user provided)
    let handler: Handler = init_handler();

    // Active window and active frame
    let (mut awin, mut aframe): (Window, Frame) = startup(&term, &handler)?;

    let mut clean = true;     // We haven't interrupted yet
    let mut act = DoNo;       // Default action is to do nothing

    // If no interrupts
    while clean {                               
	act =  awin.handle_keypress()?;         // Get next action from user input
	clean = awin.execute(act)?;	        // Handle actions
    }

    // TODO: Separate into navigation and edit modules

    // Exit successfully
    Ok(()) 
}
