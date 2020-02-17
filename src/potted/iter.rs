use super::{Pot,Node};
use crate::rust::*;

#[derive(Debug)]
pub struct Iter<'a, T:'a> {
    head   : usize,
    len    : usize,
    fr_len : usize, // forest len
    pot    : Pot<T>,
    mark   : PhantomData<&'a Node<T>>,
}

impl<'a, T:'a> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    #[inline] fn next( &mut self ) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let index = self.head;
            let pot = self.pot;
            if pot.is_forest( index ) {
                self.fr_len = pot.degree( index );
                self.head = pot.tail( self.head );
                return Some( unsafe{ transmute( &pot[ self.head ])});
            } else if self.fr_len != 0 {
                self.fr_len -= 1;
                if self.fr_len == 0 {
                    self.head = pot.parent( self.head );
                }
            }
            self.advance();
            Some( unsafe{ transmute( &pot[ index ])})
        }
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) { ( self.len, Some( self.len ))}
}

impl<'a,T> ExactSizeIterator for Iter<'a, T> {}
impl<'a,T> FusedIterator for Iter<'a, T> {}

impl<'a, T:'a> Iter<'a, T> {
    #[inline] pub(crate) fn new( head: usize, len: usize, pot: Pot<T> ) -> Self {
        Iter{ head, len, fr_len: 0, pot, mark: PhantomData }
    }

    #[inline] fn advance( &mut self ) { self.head = self.pot[ self.head ].next(); }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T:'a> {
    head   : usize,
    len    : usize,
    fr_len : usize, // forest len
    pot    : Pot<T>,
    mark   : PhantomData<&'a mut Node<T>>,
}

impl<'a, T:'a> IterMut<'a, T> {
    #[inline] pub(crate) fn new( head: usize, len: usize, pot: Pot<T> ) -> Self {
        IterMut{ head, len, fr_len: 0, pot, mark: PhantomData }
    }

    #[inline] fn advance( &mut self ) { self.head = self.pot[ self.head ].next(); }
}

impl<'a, T:'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut Node<T>;

    #[inline] fn next( &mut self ) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let index = self.head;
            let mut pot = self.pot;
            if pot.is_forest( index ) {
                self.fr_len = pot.degree( index );
                self.head = pot.tail( self.head );
                return Some( unsafe{ transmute( &mut pot[ self.head ])});
            } else if self.fr_len != 0 {
                self.fr_len -= 1;
                if self.fr_len == 0 {
                    self.head = pot.parent( self.head );
                }
            }
            self.advance();
            Some( unsafe{ transmute( &mut pot[ index ])})
        }
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) { ( self.len, Some( self.len ))}
}

impl<'a,T> ExactSizeIterator for IterMut<'a, T> {}
impl<'a,T> FusedIterator for IterMut<'a, T> {}
