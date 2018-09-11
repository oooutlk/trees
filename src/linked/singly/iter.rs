use super::{Node,Link};
use rust::*;

/// An iterator over the sub `Node`s of a `Node` or `Forest`.
///
/// This `struct` is created by [`Node::iter`] and [`Forest::iter`].
/// See its document for more.
///
/// [`Node::iter`]: struct.Node.html#method.iter
/// [`Forest::iter`]: struct.Forest.html#method.iter
pub struct Iter<'a, T:'a> {
    head : *const Link,
    tail : *const Link,
    mark : PhantomData<&'a Node<T>>,
}

impl<'a, T:'a> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    #[inline] fn next( &mut self ) -> Option<&'a Node<T>> {
        if self.head.is_null() {
             None
        } else { unsafe {
            let node = self.head;
            self.head = if self.head == self.tail {
                null()
            } else {
                (*node).next
            };
            Some( &*( node as *mut Node<T> ))
        }}
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) {
        if self.head.is_null() {
            ( 0, Some(0) )
        } else {
            let mut len = 1_usize;
            let mut head = self.head;
            while head != self.tail {
                unsafe{ head = (*head).next; }
                len += 1;
            }
            ( len, Some( len ))
        }
    }
}

impl<'a,T> ExactSizeIterator for Iter<'a, T> {}
impl<'a,T> FusedIterator for Iter<'a, T> {}

impl<'a, T:'a> Iter<'a, T> {
    #[inline] pub(crate) fn new( head: *const Link, tail: *const Link ) -> Self {
        Iter{ head, tail, mark: PhantomData }
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

/// A mutable iterator over the sub `Node`s of a `Node` or `Forest`.
///
/// This `struct` is created by [`Node::iter_mut`] and [`Forest::iter_mut`].
///  See its document for more.
///
/// [`Node::iter_mut`]: struct.Node.html#method.iter_mut
/// [`Forest::iter_mut`]: struct.Forest.html#method.iter_mut
pub struct IterMut<'a, T:'a> {
    head : *mut Link,
    tail : *mut Link,
    mark : PhantomData<&'a mut Node<T>>,
}

impl<'a, T:'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut Node<T>;

    #[inline] fn next( &mut self ) -> Option<&'a mut Node<T>> {
        if self.head.is_null() {
             None
        } else { unsafe {
            let node = self.head;
            self.head = if self.head == self.tail {
                null_mut()
            } else {
                (*node).next
            };
            Some( &mut *( node as *mut Node<T> ))
        }}
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) {
        if self.head.is_null() {
            ( 0, Some(0) )
        } else {
            let mut len = 1_usize;
            let mut head = self.head;
            while head != self.tail {
                unsafe{ head = (*head).next; }
                len += 1;
            }
            ( len, Some( len ))
        }
    }
}

impl<'a,T> ExactSizeIterator for IterMut<'a, T> {}
impl<'a,T> FusedIterator for IterMut<'a, T> {}

impl<'a, T:'a> IterMut<'a, T> {
    #[inline] pub(crate) fn new( head: *mut Link, tail: *mut Link ) -> Self {
        IterMut{ head, tail, mark: PhantomData }
    }
}

unsafe impl<'a, T:Sync> Send for Iter<'a, T> {}
unsafe impl<'a, T:Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T:Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T:Sync> Sync for IterMut<'a, T> {}
