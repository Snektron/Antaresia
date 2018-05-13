use std::default::Default;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    fn default() -> Self {
        Location {
            line: 0,
            col: 0
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span(pub Location, pub Location);

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for Span {
    fn default() -> Self {
        Span(Default::default(), Default::default())
    }
}