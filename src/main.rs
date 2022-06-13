/* Author: Garrett Hill */

use console::Term;
use std::io::Result;

// Use windows and frames in main functionality
mod gaymacs;

use crate::gaymacs::window::{Window, init_win};
use crate::gaymacs::actions::{Action, Action::*};

// Core functionality
fn main() -> Result<()> {
    // Terminal abstraction for easier management of terminal interaction
    let term: Term = Term::stdout();
    term.clear_screen()?;
    term.show_cursor()?;

    // Active window, interrupt flag
    let mut awin: Window = init_win(&term);
    let mut clean = true;

    // While no interrupts
    while clean {
	let _out: bool  = awin.refresh()?;         // Update the screen
	let act: Action = awin.handle_keypress()?; // Get next action from user
	clean = awin.execute(act)?;	           // Handle actions	
    }

    // Exit successfully
    Ok(()) 
}
