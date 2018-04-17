use std::fmt::Display;
use std::fmt;

pub fn write_comma_seperated<I>(f: &mut fmt::Formatter, it: I) -> fmt::Result
where I: Iterator,
      I::Item: Display {
    let mut first = true;

    for i in it {
        if first {
            first = false;
        } else {
            write!(f, ", ")?;
        }

        i.fmt(f)?;
    }

    Ok(())
}