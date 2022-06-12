use std::io::Result;
use console::Term;

pub struct Modeline {
    buf: String,
    term: Term,
}

impl Modeline {
    pub fn init_modeline(t: Term) -> Modeline {
	Modeline {
	    buf:  String::from("Modeline"),
	    term: t,
	}
    }

    pub fn print(&self) -> Result<bool> {
	self.term.write_line(&self.buf)?;
	Ok(true)
    }
}
