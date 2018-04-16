use std::iter::Iterator;
use std::default::Default;

pub struct Scoped<'a, T>
where T: 'a {
    next: Option<&'a Scoped<'a, T>>,
    item: T
}

impl<'a, T> Scoped<'a, T> {
    pub fn new(item: T) -> Self {
        Scoped {
            next: None,
            item
        }
    }

    pub fn enter_with<'b>(&'b self, item: T) -> Scoped<'b, T> {
        Scoped {
            next: Some(self),
            item
        }
    }

    pub fn item(&self) -> &T {
        &self.item
    }

    pub fn item_mut(&mut self) -> &mut T {
        &mut self.item
    }

    pub fn iter<'b>(&'b self) -> Iter<'b, T> {
        Iter {
            current: Some(self)
        }
    }
}

impl<'a, T> Scoped<'a, T>
where T: Default {
    pub fn enter<'b>(&'b self) -> Scoped<'b, T> {
        self.enter_with(Default::default())
    }
}

pub struct Iter<'a, T>
where T: 'a {
    current: Option<&'a Scoped<'a, T>>
}

impl<'a, T> Iterator for Iter<'a, T>
where T: 'a {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.current = current.next;
            Some(&current.item)
        } else {
            None
        }
    }
}