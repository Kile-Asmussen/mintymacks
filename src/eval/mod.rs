use std::collections::HashMap;

use crate::{bits::board::BitBoard, zobrist::ZobHash};

pub trait Evaluator {
    fn stateval(&mut self, board: &BitBoard);
}

pub enum StackList<'a, T, U> {
    Link(T, &'a StackList<'a, T, U>),
    Base(U),
}

impl<'a, T, U> StackList<'a, T, U> {
    pub fn new(u: U) -> Self {
        Self::Base(u)
    }

    pub fn push<'b>(&'a self, t: T) -> StackList<'b, T, U>
    where
        'a: 'b,
    {
        StackList::Link(t, self)
    }

    pub fn tail(&self) -> &U {
        match self {
            StackList::Link(_, next) => next.tail(),
            StackList::Base(x) => x,
        }
    }
}

impl<'a, T> StackList<'a, T, T> {
    pub fn head(&self) -> &T {
        match self {
            StackList::Link(x, _) => x,
            StackList::Base(x) => x,
        }
    }
}
