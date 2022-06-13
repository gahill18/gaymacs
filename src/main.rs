/* Author: Garrett Hill */

use console::Term;
use std::io::Result;

// Use windows and frames in main functionality
mod gaymacs;

use crate::gaymacs::window::{Window, init_win};
use crate::gaymacs::actions::{Action, Action::*};

// Core functionality
fn main() -> Result<()> {
    // TERMinal abstraction for easier management of terminal interaction
    let term: Term = Term::stdout();
    term.clear_screen()?;
    term.show_cursor()?;

    // Initialize our active window and default next action
    let mut awin: Window = init_win(&term);
    let mut next_action: Action = DoNo;

    // While no interrupts
    while !awin.is_interrupted() {	
	awin.refresh()?;                           // Update the screen	
	next_action = awin.handle_keypress()?;     // Get next action from user
	awin.execute(next_action)?;                // Handle actions
    }

    // Exit successfully
    Ok(()) 
}
