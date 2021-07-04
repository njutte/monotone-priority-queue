//! A monotone priority queue
//! https://en.wikipedia.org/wiki/Monotone_priority_queue

use std::vec;
use std::slice;

pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

pub struct MonotonePriorityQueue<T> {
    data: Vec<T>,
}

impl<T> MonotonePriorityQueue<T>
where T: Ord,
{
    pub fn new() -> MonotonePriorityQueue<T> {
        MonotonePriorityQueue { data: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> MonotonePriorityQueue<T> {
        MonotonePriorityQueue { data: Vec::with_capacity(capacity) }
    }

    // pub fn peek_mut(&mut self) -> Option<PeekMut<'_, T>> {
    //     todo!()
    // }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn push(&mut self, item: T) {
        todo!()
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        self.into_iter_sorted().collect()
    }

    pub fn append(&mut self, other: &mut MonotonePriorityQueue<T>) {
        todo!()
    }

    // pub fn drain_sorted(&mut self) -> DrainSorted<'_, T>

    // pub fn retain<F>(&mut self, f: F)
    // where F: FnMut(&T) -> bool
}

impl<T> MonotonePriorityQueue<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { iter: self.data.iter() }
    }

    pub fn into_iter_sorted(self) -> IntoIterSorted<T> {
        IntoIterSorted { inner: self }
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }

    pub fn reserve(&mut self, additional: usize) {
        todo!()
    }

    pub fn shrink_to_fit(&mut self) {
        todo!()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        todo!()
    }

    // pub fn as_slice(&self) -> &[T]

    pub fn into_vec(self) -> Vec<T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn drain(&mut self) -> Drain<'_, T> {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }


}

pub struct IntoIterSorted<T> {
    inner: MonotonePriorityQueue<T>,
}

impl<T: Ord> Iterator for IntoIterSorted<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

pub struct Drain<'a, T: 'a> {
    iter: vec::Drain<'a, T>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
