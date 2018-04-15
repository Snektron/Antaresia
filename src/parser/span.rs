use std::default::Default;
use std::fmt;

#[derive(Clone)]
pub struct Location {
    line: usize,
    col: usize
}

impl Location {
    pub fn next(&mut self) {
        self.col += 1;
    }

    pub fn next_line(&mut self) {
        self.col = 0;
        self.line += 1;
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.line + 1, self.col + 1)
    }
}

impl Default for Location {
    fn default() -> Location {
        Location {
            line: 0,
            col: 0
        }
    }
}
