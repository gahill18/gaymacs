use std::io::Result;

#[derive(Debug,Clone)]
pub struct Killring {
    hist: Vec<String>,      // Stack for past yanks/kills	
}

impl Killring {
    // Return the default empty keyring
    pub fn init_killring() -> Killring {
	let h: Vec<String> = vec![];
	Killring {
	    hist: h,
	}	
    }

    // Store the killed text in history
    pub fn kill(&mut self, s: String) -> Result<bool> {
	self.hist.push(s);
	Ok(true)
    }

    pub fn yank(&mut self) -> Result<String> {
	match self.hist.pop() {
	    Some(s) => Ok(s),
	    None    => Ok(String::from(""))
	}
    }
}
