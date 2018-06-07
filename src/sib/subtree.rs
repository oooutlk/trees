//! A full functional mutable iterator implementation with the extra ability of inserting/removing `Node` at any position than `IterMut`.

use super::{Node,Tree};
use rust::*;

/// Wrapper of `Node` for allowing modification of parent or sib links.
/// Any `Node` that is the root of some `Tree` is impossible to be `Subtree`.
pub struct Subtree<'a, T:'a>{
    node : &'a mut Node<T>,
    prev : *mut Node<T>,
    sub : *mut *mut Node<T>,
}

impl<'a, T:'a> Subtree<'a,T> {
    /// Insert sib tree before `self`.
    /// The newly inserted node will not be iterated over by the currently running iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for mut sub in tree.subtrees() { sub.insert_before( tr(3) ); }
    /// assert_eq!( tree.to_string(), "0( 3 1 3 2 )" );
    /// ```
    #[inline] pub fn insert_before( &mut self, mut sib: Tree<T> ) {
        unsafe {
            sib.root_mut().sib = self.node as *mut Node<T>;
            (*self.prev).sib = sib.root_mut();
        }
        sib.clear();
    }

    /// Insert sib tree after `self`.
    /// The newly inserted node will not be iterated over by the currently running iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for mut sub in tree.subtrees() { sub.insert_after( tr(3) ); }
    /// assert_eq!( tree.to_string(), "0( 1 3 2 3 )" );
    /// ```
    #[inline] pub fn insert_after( &mut self, mut sib: Tree<T> ) {
        unsafe {
            (*sib.root_mut()).sib = self.node.sib;
            self.node.sib = sib.root_mut();
            if (*self.sub) == self.node as *mut Node<T> {
                *self.sub = sib.root_mut();
            }
        }
        sib.clear();
    }

    /// The subtree departs from its parent and becomes an indepent `Tree`.
    ///
    /// # Examples
    /// ```
    /// use trees::{tr,fr};
    ///
    /// let mut forest = -tr(1)-tr(2)-tr(3);
    /// //for sub in forest.subtrees() { sub.depart(); }
    /// //forest.subtrees().next().unwrap().depart();
    /// //assert_eq!( forest, fr() );
    /// ```
    #[inline] pub fn depart( self ) -> Tree<T> {
        unsafe {
            if (*self.sub) == self.node as *mut Node<T> {
                *self.sub = if self.node.has_no_sib() {
                    null_mut()
                } else {
                    self.prev
                }
            }
            (*self.prev).sib = self.node.sib;
            self.node.reset_sib();
            Tree::from( self.node as *mut Node<T> )
        }
    }
}

impl<'a, T:'a> Deref for Subtree<'a,T> {
    type Target = Node<T>;
    fn deref( &self ) -> &Node<T> { self.node }
}

impl<'a, T:'a> DerefMut for Subtree<'a,T> { fn deref_mut( &mut self ) -> &mut Node<T> { self.node }}

/// Mutable iterator allowing modification of parent or sib links.
pub struct SubtreeIter<'a, T:'a>{
    pub(crate) next : *mut Node<T>,
    pub(crate) curr : *mut Node<T>,
    pub(crate) prev : *mut Node<T>,
    pub(crate) tail : *mut Node<T>,
    pub(crate) sub : *mut *mut Node<T>,
    pub(crate) mark : PhantomData<&'a mut Node<T>>,
}

impl<'a, T:'a> Iterator for SubtreeIter<'a,T> {
    type Item = Subtree<'a,T>;

    #[inline] fn next( &mut self ) -> Option<Subtree<'a,T>> {
        if !self.tail.is_null() {
            if !self.curr.is_null() {
                if self.curr == self.tail || self.curr == self.next {
                    return None;
                }
                unsafe { 
                    if (*self.prev).sib != self.next { 
                        self.prev = self.curr; // curr did not depart()-ed
                    }
                }
            }
            self.curr = self.next;
            if !self.next.is_null() {
                let curr = self.next;
                unsafe { 
                    self.next = (*curr).sib;
                    return Some( Subtree{ node: &mut *curr, prev: self.prev, sub: self.sub });
                }
            }
        }
        None
    }
}
