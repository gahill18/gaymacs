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
// Returns the initialized starting window
fn startup(term: Term, handler: Handler) -> Result<Window> {
    // Frame Name and Initial Buffer Text
    let fname: String = String::from("*scratch*");  
    let fibuf: String = String::from("");

    // Scratch buffer and starting window
    let aframe: Frame = init_frame(0, fname, fibuf, None, term.clone());
    let mut init_win: Window = init_win(aframe.clone(), term, handler);

    // Return successfully
    Ok(init_win)     
}



// Core functionality
fn main() -> Result<()> {
    // Terminal abstraction for easier management of terminal interaction
    let term = Term::stdout();
    term.clear_screen()?;
    term.show_cursor()?;

    // Event handler, active window, interrupt flag
    let handler: Handler = init_handler();
    let mut awin: Window = startup(term, handler)?;
    let mut clean = true;

    // If no interrupts
    while clean {
	let _ = awin.refresh()?;                // Update the screen
	let act =  awin.handle_keypress()?;     // Get next action from user input
	clean = awin.execute(act)?;	        // Handle actions	
    }

    // Refresh and exit successfully
    let _ = awin.refresh()?;                
    Ok(()) 
}
