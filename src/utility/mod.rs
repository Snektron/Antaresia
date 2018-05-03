pub mod scoped;

pub use utility::scoped::Scoped;

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

pub trait JoinExt<T> {
    type Other;
    type Output;

    fn join(self, other: Self::Other) -> Self::Output
    where Self: Sized {
        self.join_with(|| other)
    }

    fn join_with<F>(self, other: F) -> Self::Output
    where F: FnOnce() -> Self::Other;
}

impl<T, U> JoinExt<U> for Option<T> {
    type Other = Option<U>;
    type Output = Option<(T, U)>;

    fn join_with<F>(self, other: F) -> Self::Output
    where F: FnOnce() -> Self::Other {
        self.and_then(|x| other().map(|y| (x, y)))
    }
}

impl<T, U, E> JoinExt<U> for Result<T, E> {
    type Other = Result<U, E>;
    type Output = Result<(T, U), E>;

    fn join_with<F>(self, other: F) -> Self::Output
    where F: FnOnce() -> Self::Other {
        self.and_then(|x| other().map(|y| (x, y)))
    }
}

pub trait TestExt<T, E> {
    fn test<P>(self, pred: P, err: E) -> Result<T, E>
    where Self: Sized,
          P: FnOnce(&T) -> bool {
        self.test_or(pred, |_| err)
    }

    fn test_or<P, F>(self, pred: P, err: F) -> Result<T, E>
    where P: FnOnce(&T) -> bool,
          F: FnOnce(T) -> E;
}

impl<T, E> TestExt<T, E> for Result<T, E> {
    fn test_or<P, F>(self, pred: P, err: F) -> Result<T, E>
    where P: FnOnce(&T) -> bool,
          F: FnOnce(T) -> E {
        self.and_then(|x| {
            if pred(&x) {
                Ok(x)
            } else {
                Err(err(x))
            }
        })
    }
}