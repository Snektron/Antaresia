use std::iter::Iterator;

struct Scoped<T> {
    prev: Option<Box<Scoped<T>>>,
    inner: T
}

impl Scoped<T> {
    pub fn new() -> Self {
        Scoped {
            prev: None
        }
    }

    pub fn enter(self) -> Self {
        Scoped {
            prev: Some(self)
        }
    }

    pub fn exit(self) -> Self {
        self.prev
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {

    }
}

struct Iter<'a, T> {
    current: Option<&'a Scoped<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.current = &current.prev;
            Some(current)
        } else {
            None
        }
    }
}