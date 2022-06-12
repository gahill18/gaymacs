#[derive(Debug,Clone)]
pub struct Killring {
    hist: Vec<String>,      // Stack for past yanks/kills	
}

impl Killring {
    pub fn init_killring() -> Killring {
	Killring {
	    hist: Vec::new(),
	}	
    }
}
