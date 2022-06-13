use std::io::Result;
use console::Term;

pub struct Modeline<'a> {
    buf: String,
    term: &'a Term,
}

impl <'a> Modeline<'a> {
    pub fn init_modeline(t: &'a Term) -> Modeline<'a> {
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
