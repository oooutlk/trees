// Copyright 2018 oooutlk@outlook.com. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![feature( alloc )]
#![feature( box_into_raw_non_null )]

//! # trees
//!
//! Provides the `Tree`/`Forest` data structures which are suitable for storing hierarchical data built from the bottom up.
//! 
//! This crate does not depend on libstd, and can be regarded as the nonlinear version of std::collections::LinkedList.
//!
//! ## Quick start
//!
//! 1. `Tree` notation
//!
//! ```rust,no_run
//! use trees::tr;      // tr stands for tree
//! tr(0);              // A single tree node with data 0. tr(0) has no children
//! tr(0) /tr(1);       // tr(0) has one child tr(1)
//! tr(0) /tr(1)/tr(2); // tr(0) has children tr(1) and tr(2)
//! 
//! // tr(0) has children tr(1) and tr(4), while tr(1) has children tr(2) and tr(3), and tr(4) has children tr(5) and tr(6).
//! // The spaces and carriage returns are for pretty format and do not make sense.
//! tr(0)
//!     /( tr(1) /tr(2)/tr(3) )
//!     /( tr(4) /tr(5)/tr(6) );
//! ```
//!
//! 2. `Forest` notation
//!
//! ```rust,no_run
//! use trees::{tr,fr}; // fr stands for forest
//!
//! fr::<i32>();        // An empty forest
//! fr() - tr(1);       // forest has one child tr(1)
//! - tr(1);            // forest has one child tr(1). The fr() can be omitted. The Neg operator for Tree converts the tree to a forest.
//! - tr(1) - tr(2);    // forest has child tr(1) and tr(2)
//! tr(1) - tr(2);      // forest has child tr(1) and tr(2). The leading neg can be omitted.
//! 
//! // forest has children tr(1) and tr(4), while tr(1) has children tr(2) and tr(3), and tr(4) has children tr(5) and tr(6).
//! -( tr(1) /tr(2)/tr(3) )
//! -( tr(4) /tr(5)/tr(6) );
//!
//! // A tree tr(0) whose children equal to the forest descripted above.
//! tr(0) /(
//!     -( tr(1) /( -tr(2)-tr(3) ) )
//!     -( tr(4) /( -tr(5)-tr(6) ) )
//! );
//! ```
//!
//! 3. Preorder traversal
//!
//! ```rust
//! use std::string::{String,ToString};
//! use trees::{tr,Node};
//!
//! let tree = tr(0)
//!     /( tr(1) /tr(2)/tr(3) )
//!     /( tr(4) /tr(5)/tr(6) );
//!
//! fn tree_to_string<T:ToString>( node: &Node<T> ) -> String {
//!     if node.is_leaf() {
//!         node.data.to_string()
//!     } else {
//!         node.data.to_string()
//!             + &"( "
//!             + &node.children()
//!                 .fold( String::new(),
//!                     |s,c| s + &tree_to_string(c) + &" " )
//!             + &")"
//!     }
//! }
//!
//! assert_eq!( tree_to_string( &tree ), "0( 1( 2 3 ) 4( 5 6 ) )" );
//! ```
//! 
//! 4. String representation 
//! 
//! The `Debug` and `Display` trait has been implemented and are essentially the same as tree_to_tring() mentioned above.
//!
//! Children are seperated by spaces and grouped in the parentheses that follow their parent closely. 
//! 
//! ```rust
//! use trees::{tr,fr};
//!
//! let tree = tr(0) /( tr(1) /tr(2)/tr(3) ) /( tr(4) /tr(5)/tr(6) );
//! let str_repr = "0( 1( 2 3 ) 4( 5 6 ) )";
//! assert_eq!( tree.to_string(), str_repr );
//! assert_eq!( format!( "{:?}", tree ), str_repr );
//! 
//! assert_eq!( fr::<i32>().to_string(), "()" );
//! assert_eq!( format!( "{:?}", fr::<i32>() ), "()" );
//! 
//! let forest = -( tr(1) /tr(2)/tr(3) ) -( tr(4) /tr(5)/tr(6) );
//! let str_repr = "( 1( 2 3 ) 4( 5 6 ) )";
//! assert_eq!( forest.to_string(), str_repr );
//! assert_eq!( format!( "{:?}", forest ), str_repr );
//! ```
//!
//! ## Slow start
//!
//! ### Concepts
//!
//! 1. `Tree` is composed of a root `Node` and an optional `Forest` as its children. A tree can NOT be empty.
//! ```rust,no_run
//! use trees::{tr,Tree,Forest};
//!
//! let tree: Tree<i32> = tr(0);
//! let forest: Forest<i32> = -tr(1)-tr(2)-tr(3);
//! let mut tree = tree.adopt( forest );
//! let forest = tree.abandon();
//! ```
//! 2. `Forest` is composed of `Node`s as its children. A forest can be empty.
//! ```rust,no_run
//! use trees::{tr,fr,Tree,Forest};
//!
//! let mut forest: Forest<i32> = fr(); // an empty forest
//! forest.push_back( tr(1) );          // forest has one tree
//! forest.push_back( tr(2) );          // forest has two trees
//! ```
//! 3. `Node` is a borrowed tree, and `Tree` is an owned `Node`. All nodes in a tree can be referenced as `&Node`, but only the root node can be observed as `Tree` by the user.
//! ```rust,no_run
//! use trees::{tr,Tree,Node};
//! use std::borrow::Borrow;
//!
//! let mut tree: Tree<i32>  = tr(0) /tr(1)/tr(2)/tr(3);
//! {
//!     let root: &Node<i32> = tree.borrow();
//!     let first_child : &Node<i32> = tree.children().next().unwrap();
//!     let second_child: &Node<i32> = tree.children().nth(2).unwrap();
//!     let third_child : &Node<i32> = tree.children().last().unwrap();
//! }
//! let first_child: Tree<i32> = tree.pop_front().unwrap();
//! ```
//! ### Iterators
//!
//! The children nodes of a node, or a forest, is conceptually a forward list.
//! 
//! 1. Using `children()` to iterate over referenced child `Node`s, you can:
//! 
//! * read the data associated with each node.
//! 
//! * using `children()` to iterate over children's children, perhaps read the data associated with children's children, etc.
//! 
//! 2. Using `children_mut()` to iterate over referenced child `Node`s, you can:
//! 
//! * read/write the data associated with each node, or `adopt()`, `abandon()`, `push_front()`, `pop_front()`, `push_back()` child node(s) in constant time.
//! 
//! * using `children_mut()` to iterate over children's children, perhaps read/write the data associated with children's children, or `adopt()`, `abandon()`, `push_front()`, `pop_front()`, `push_back()` child node(s) in constant time, etc.
//! 
//! 3. Using `subtrees()` to iterate over `Subtree`s, you can:
//! 
//! * `insert_sib()`, `remove()` node(s) at given position in the children forward list in O(n) time.
//! 
//! * do whatever `children()` or `children_mut()` allows to do.
//! 
//! 4. Using `Forest::<T>::into_iter()` to iterate over `Tree`s, you can:
//! 
//! * do whatever you want to.
//! 
//! ### Resource management
//!
//! 1. `Tree`/`Forest` are implemented in extrusive manner with two extra pointers per node, and will recursively destruct all the nodes owned by the tree/forest when reaching the end of their lifetimes.
//! 2. `Clone` for `Tree` and `Forest` makes deep copy which clones all its decendant nodes. To do copy for just one node, simplely `let cloned = trees::tr( node.data.clone() );`.
//! 3. No bookkeeping of size information. 
//!
//! ### Panics
//!
//! No panics unless `Clone` is involved:
//! * `Node::<T>::to_owned()`
//! * `Tree::<T>::clone()`
//! *  `Forest::<T>::clone()`
//! *  all of the operator overloading functions the operands of which contain at least one referenced type.
//!
//! Panics if and only if `T::clone()` panics.

extern crate alloc;

use alloc::borrow::{Borrow,BorrowMut,ToOwned};
use alloc::boxed::Box;
use core::cmp::Ordering;
use core::cmp::Ordering::*;
use core::fmt;
use core::fmt::{Debug,Display,Formatter};
use core::hash::{Hasher,Hash};
use core::iter::FromIterator;
use core::marker::PhantomData;
use core::ops::{Deref,DerefMut,Div,Neg,Sub};
use core::ptr::NonNull;

/// Tree node implemented in singly-linked-children / circularly-linked-siblings.
pub struct Node<T> {
    next_sib : Option<NonNull<Node<T>>>,
    last_chd : Option<NonNull<Node<T>>>,
    pub data : T,
    marker   : PhantomData<Box<Node<T>>>,
}

/// Collection of circularly-linked `Tree`s
pub struct Forest<T> {
    tail   : Option<NonNull<Node<T>>>,
    marker : PhantomData<Box<Node<T>>>,
}

/// Tree with owned `Node`s.
#[derive( PartialEq, Eq )]
pub struct Tree<T>( Box<Node<T>> );

/// An iterator over the direct decendants of a tree `Node` or `Forest`.
///
/// This `struct` is created by the [`children`] method on [`Tree`] and the [`children`] method on [`Forest`]. See its
/// documentation for more.
///
/// [`children`]: struct.Node.html#method.children
/// [`Tree`]: struct.Node.html
/// [`children`]: struct.Forest.html#method.children
/// [`Forest`]: struct.Forest.html
pub struct Iter<'a, T:'a> {
    head   : Option<NonNull<Node<T>>>,
    tail   : Option<NonNull<Node<T>>>,
    marker : PhantomData<&'a Node<T>>,
}

impl<'a, T:'a> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    #[inline]
    fn next( &mut self ) -> Option<&'a Node<T>> {
        self.head.map( |node| unsafe {
            let node = node.as_ptr();
            self.head = if self.head == self.tail {
                None
            } else {
                (*node).next_sib
            };
            &*node
        })
    }
}

/// A mutable iterator over the direct decendants of a tree `Node` or `Forest`.
///
/// This `struct` is created by the [`children_mut`] method on [`Tree`] and the [`children_mut`] method on [`Forest`]. See its
/// documentation for more.
///
/// [`children_mut`]: struct.Node.html#method.children_mut
/// [`Node`]: struct.Node.html
/// [`children_mut`]: struct.Forest.html#method.children_mut
/// [`Forest`]: struct.Forest.html
pub struct IterMut<'a, T: 'a> {
    head   : Option<NonNull<Node<T>>>,
    tail   : Option<NonNull<Node<T>>>,
    marker : PhantomData<&'a Node<T>>,
}

impl<'a, T:'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut Node<T>;

    #[inline]
    fn next( &mut self ) -> Option<&'a mut Node<T>> {
        self.head.map( |node| unsafe {
            let node = node.as_ptr();
            self.head = if self.head == self.tail {
                None
            } else {
                (*node).next_sib
            };
            &mut *node
        })
    }
}

/// Wrapper of tree `Node` with the additional function of inserting/removing node at given position in the subtrees in O(n) time.
pub struct Subtree<T> {
    curr   : NonNull<Node<T>>,
    prev   : NonNull<Node<T>>,
    ptail  : *mut Option<NonNull<Node<T>>>,
    marker : PhantomData<Box<Node<T>>>,
}

impl<T> Subtree<T> {
    /// Insert the sib node after `self` node.
    /// The newly inserted node will not be iterated over by the currently running iterator.
    ///
    /// # Examples
    ///
    /// ## insert after
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut forest = tr(1)-tr(2)-tr(3);
    /// for mut sub in forest.subtrees() {
    ///         sub.insert_sib( tr(3) );
    /// }
    /// assert_eq!( forest, tr(1)-tr(3)-tr(2)-tr(3)-tr(3)-tr(3) );
    /// ```
    ///
    /// ## insert before
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut forest = tr(1)-tr(3)-tr(4);
    /// let mut iter = forest.subtrees().peekable();
    /// while let Some(mut sub) = iter.next() { 
    ///     if let Some(next_sub) = iter.peek() {
    ///         if next_sub.data == 3 {
    ///             sub.insert_sib( tr(2) );
    ///         }
    ///     }
    /// }
    /// assert_eq!( forest, tr(1)-tr(2)-tr(3)-tr(4) );
    /// ```
    pub fn insert_sib( &mut self, sib: Tree<T> ) {
        unsafe {
            let sib = Box::into_raw_non_null( sib.0 );
            let curr = self.curr.as_ptr();
            (*sib.as_ptr()).next_sib = (*curr).next_sib;
            (*curr).next_sib = Some(sib); 
            if (*self.ptail) == Some(self.curr) {
                (*self.ptail) = Some(sib);
            }
        }
    }

    /// This subtree removes it`self` from its parent tree
    ///
    /// # Examples
    /// ```
    /// use trees::{tr,fr};
    ///
    /// let mut forest = tr(1)-tr(2)-tr(3)-tr(4)-tr(5)-tr(6);
    /// for sub in forest.subtrees() { sub.remove(); }
    /// assert_eq!( forest, fr() );
    /// ```
    pub fn remove( self ) -> Tree<T> {
        unsafe {
            if (*self.ptail) == Some(self.curr) {
                (*self.ptail) =
                    if self.prev == self.curr {
                        None // the only child is about to be removed
                    }
                    else {
                        Some(self.prev)
                    };
            }
            let prev = self.prev.as_ptr();
            let curr = self.curr.as_ptr();
            (*prev).next_sib = (*curr).next_sib;
            (*curr).next_sib = None;
            Tree( Box::from_raw( self.curr.as_ptr() ))
        }
    }
}

impl<T> Deref for Subtree<T> {
    type Target = Node<T>;

   fn deref( &self ) -> &Node<T> {
        unsafe { self.curr.as_ref() }
   }
}

impl<T> DerefMut for Subtree<T> {
    fn deref_mut( &mut self ) -> &mut Node<T> {
        unsafe { self.curr.as_mut() }
    }
}

/// A mutable iterator over the direct decendants of a `Tree` or `Forest`.
///
/// This `struct` is created by the [`subtrees`] method on [`Tree`] and [`Forest`]. See its
/// documentation for more.
///
/// [`subtrees`]: struct.Node.html#method.subtrees
/// [`Node`]: struct.Node.html
/// [`subtrees`]: struct.Forest.html#method.subtrees
/// [`Forest`]: struct.Forest.html
pub struct SubtreeIter<T> {
    next   : Option<NonNull<Node<T>>>,
    curr   : Option<NonNull<Node<T>>>,
    prev   : Option<NonNull<Node<T>>>,
    tail   : Option<NonNull<Node<T>>>,
    ptail  : *mut Option<NonNull<Node<T>>>,
    marker : PhantomData<Box<Node<T>>>,
}

impl<T> Iterator for SubtreeIter<T> {
    type Item = Subtree<T>;

    #[inline]
    fn next( &mut self ) -> Option<Subtree<T>> {
        unsafe {
            if let Some(tail) = self.tail {
                if let Some(curr) = self.curr {
                    if curr == tail || self.curr == self.next {
                        return None;
                    }
                    if (*self.prev.unwrap().as_ptr()).next_sib != self.next { 
                        self.prev = self.curr; // curr is not remove()-ed
                    }
                }
                self.curr = self.next;
                if let Some(curr) = self.next {
                    self.next = (*curr.as_ptr()).next_sib;
                    return Some( Subtree {
                        curr   : curr,
                        prev   : self.prev.unwrap(),
                        ptail  : self.ptail,
                        marker : PhantomData,
                    });
                }
            }
        }
        None
    }
}

/// An owning iterator over the children of a `Forest`.
///
/// This `struct` is created by the [`into_iter`] method on [`Forest`]. See its documentation for more.
///
/// [`into_iter`]: struct.Forest.html#method.into_iter
/// [`Forest`]: struct.Forest.html
pub struct IntoIter<T> {
    forest : Forest<T>,
    marker : PhantomData<Tree<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = Tree<T>;

    #[inline]
    fn next( &mut self ) -> Option<Tree<T>> {
        self.forest.pop_front()
    }
}

impl<T> IntoIterator for Forest<T> {
    type Item = Tree<T>;
    type IntoIter = IntoIter<T>;

    /// Consumes the `Forest` into an iterator yielding `Tree`s.
    #[inline]
    fn into_iter( self ) -> IntoIter<T> {
        IntoIter{ forest: self, marker: PhantomData }
    }
}

impl<T> FromIterator<Tree<T>> for Forest<T> {
   fn from_iter<I:IntoIterator<Item=Tree<T>>>( iter: I ) -> Self {
        let mut iter = iter.into_iter();
        let mut children = fr::<T>();
        while let Some(node) = iter.next() {
            children.push_back( node );
        }
        children
    }
}

impl<T> Extend<Tree<T>> for Node<T> {
    fn extend<I:IntoIterator<Item=Tree<T>>>( &mut self, iter: I ) {
        for child in iter.into_iter() {
            self.push_back( child );
        }
    }
}

impl<T> Extend<Tree<T>> for Forest<T> {
    fn extend<I:IntoIterator<Item=Tree<T>>>( &mut self, iter: I ) {
        for child in iter.into_iter() {
            self.push_back( child );
        }
    }
}

impl<T:PartialEq> PartialEq for Node<T> {
    fn eq( &self, other: &Self ) -> bool {
        self.data == other.data && self.children().eq( other.children() )
    }

    fn ne( &self, other: &Self ) -> bool {
        self.data != other.data || self.children().ne( other.children() )
    }
}

impl<T:Eq> Eq for Node<T> {}

impl<T:PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        match self.data.partial_cmp( &other.data ) {
            None        => None,
            Some(order) =>
                match order {
                    Less    => Some(Less),
                    Greater => Some(Greater),
                    Equal   => self.children().partial_cmp( other.children() ),
                },
        }
    }
}

impl<T:Ord> Ord for Node<T> {
    #[inline]
    fn cmp( &self, other: &Self ) -> Ordering {
        match self.data.cmp( &other.data ) {
            Less    => Less,
            Greater => Greater,
            Equal   => self.children().cmp( other.children() ),
        }
    }
}

impl<T> Borrow<Node<T>> for Tree<T> {
    fn borrow( &self ) -> &Node<T> { &self.0 }
}

impl<T> BorrowMut<Node<T>> for Tree<T> {
    fn borrow_mut( &mut self ) -> &mut Node<T> { &mut self.0 }
}

impl<T:Clone> ToOwned for Node<T> {
    type Owned = Tree<T>;
    fn to_owned( &self ) -> Tree<T> {
        tr( self.data.clone() ).adopt(
            self.children()
                .map( |child| child.to_owned() )
                .collect()
        )
    }
}

impl<T:Clone> Clone for Tree<T> {
    fn clone( &self ) -> Self {
        self.0.to_owned()
    }
}

impl<T:Clone> Clone for Forest<T> {
    fn clone( &self ) -> Self {
        self.children()
            .map( |child| child.to_owned() )
            .collect()
    }
}

impl<T:PartialEq> PartialEq for Forest<T> {
    fn eq( &self, other: &Self ) -> bool {
        self.children().eq( other.children() )
    }

    fn ne( &self, other: &Self ) -> bool {
        self.children().ne( other.children() )
    }
}

impl<T:Eq> Eq for Forest<T> {}

impl<T:PartialOrd> PartialOrd for Forest<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        self.children().partial_cmp( other.children() )
    }
}

impl<T:Ord> Ord for Forest<T> {
    #[inline]
    fn cmp( &self, other: &Self ) -> Ordering {
        self.children().cmp( other.children() )
    }
}

impl<T> Node<T> {
    /// Append the given trees at the end of the `Node`'s children list.
    #[inline]
    pub fn adopt( &mut self, children: Forest<T> ) {
        let mut children = self.abandon().merge( children );
        self.last_chd = children.tail;
        children.tail = None;
    }

    /// Removes and returns the given tree `Node`'s children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// let children = tree.abandon();
    /// assert_eq!( tree, tr(0) );
    /// assert_eq!( children, -tr(1)-tr(2) );
    /// ```
    /// 
    pub fn abandon( &mut self ) -> Forest<T> {
        let forest = Forest::<T>{ tail: self.last_chd, marker: PhantomData };
        self.last_chd = None;
        forest
    }

    /// Adds the child as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_front( tr(2) );
    /// let mut iter = tree.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(2) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn push_front( &mut self, child: Tree<T> ) {
        let children = self.abandon().prepend( child );
        self.adopt( children );
    } 

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// let child = tree.pop_front();
    /// assert_eq!( tree, tr(0) /tr(2) );
    /// assert_eq!( child.unwrap(), tr(1) );
    /// ```
    #[inline]
    pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        let mut children = self.abandon();
        let front = children.pop_front();
        self.adopt( children );
        front
    }

    /// add the child as the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_back( tr(2) );
    /// let mut iter = tree.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(2) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn push_back( &mut self, child: Tree<T> ) {
        let children = self.abandon().append( child );
        self.adopt( children );
    }

    /// Provides a forward iterator over tree `Node`'s direct decendants
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let tree = tr(0)
    ///     /( tr(1) /tr(2)/tr(3) )
    ///     /( tr(4) /tr(5)/tr(6) )
    /// ;
    /// let mut iter = tree.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) /tr(2)/tr(3) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(4) /tr(5)/tr(6) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn children( &self ) -> Iter<T> {
        unsafe {
            match self.last_chd {
                None => Iter{ head: None, tail: None, marker: PhantomData },
                Some(last_chd) => Iter{
                    head   : (*last_chd.as_ptr()).next_sib,
                    tail   : self.last_chd,
                    marker : PhantomData,
                }
            }
        }
    }

    /// Provides a forward iterator over tree `Node`'s direct decendants with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0)
    ///     /( tr(1) /tr(2)/tr(3) )
    ///     /( tr(4) /tr(5)/tr(6) )
    /// ;
    /// for mut child in tree.children_mut() {
    ///     child.data *= 10;
    /// }
    /// let expedted = tr(0)
    ///     /( tr(10) /tr(2)/tr(3) )
    ///     /( tr(40) /tr(5)/tr(6) )
    /// ;
    /// assert_eq!( tree, expedted );
    /// ```
    #[inline]
    pub fn children_mut( &mut self ) -> IterMut<T> {
        unsafe {
            match self.last_chd {
                None => IterMut{ head: None, tail: None, marker: PhantomData },
                Some(last_chd) => IterMut {
                    head   : (*last_chd.as_ptr()).next_sib,
                    tail   : self.last_chd,
                    marker : PhantomData,
                }
            }
        }
    }

    /// Provide an iterator over the tree `Node`'s subtrees for insert/remove at any position
    ///
    /// # Examples
    ///
    /// ## insert after
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0) /tr(1)/tr(2)/tr(3);
    /// for mut sub in tree.subtrees() {
    ///     sub.insert_sib( tr(3) );
    /// }
    /// assert_eq!( tree, tr(0) /tr(1)/tr(3)/tr(2)/tr(3)/tr(3)/tr(3) );
    /// ```
    ///
    /// ## insert before
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0) /tr(1)/tr(3)/tr(4);
    /// let mut iter = tree.subtrees().peekable();
    /// while let Some(mut sub) = iter.next() { 
    ///     if let Some(next_sub) = iter.peek() {
    ///         if next_sub.data == 3 {
    ///             sub.insert_sib( tr(2) );
    ///         }
    ///     }
    /// }
    /// assert_eq!( tree, tr(0) /tr(1)/tr(2)/tr(3)/tr(4) );
    /// ```
    ///
    /// ## remove
    /// ```
    /// use trees::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2)/tr(3)/tr(4)/tr(5)/tr(6);
    /// for mut sub in tree.subtrees() {
    ///     let d = sub.data;
    ///     if d%2 == 0 || d%3 == 0 {
    ///         sub.remove();
    ///     }
    /// }
    /// assert_eq!( tree, tr(0) /tr(1)/tr(5) );
    /// ```
    #[inline]
    pub fn subtrees( &mut self ) -> SubtreeIter<T> {
        unsafe {
            match self.last_chd {
                None => SubtreeIter {
                    next: None, curr: None, prev: None, tail: self.last_chd,
                    ptail: &mut self.last_chd as *mut Option<NonNull<Node<T>>>,
                    marker: PhantomData,
                },
                Some(last_chd) => SubtreeIter {
                    next   : (*last_chd.as_ptr()).next_sib,
                    curr   : None,
                    prev   : Some(last_chd),
                    tail   : self.last_chd,
                    ptail  : &mut self.last_chd as *mut Option<NonNull<Node<T>>>,
                    marker : PhantomData,
                }
            }
        }
    }

    /// Returns `true` if the `Node` has no children.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr};
    ///
    /// let mut tree = tr(0);
    /// assert!( tree.is_leaf() );
    /// tree.push_back( tr(1) ); 
    /// assert!( !tree.is_leaf() );
    /// ```
    #[inline]
    pub fn is_leaf( &self ) -> bool { self.last_chd.is_none() }

}

impl<T> Tree<T> {
    /// Creates a `Tree` with given data on heap.
    /// `Tree` is NOT nullable. Consider using an empty `Forest` instead if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::Tree;
    ///
    /// let tree = Tree::new( 1 );
    /// assert_eq!( tree.data, 1 );
    /// ```
    #[inline]
    pub fn new( data: T ) -> Tree<T> {
        Tree( Box::new( Node{
            next_sib : None,
            last_chd : None,
            data     : data,
            marker   : PhantomData,
        }))
    }

    /// Append the given trees at the end of the `Tree`'s children list.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut tree = tr(0);
    /// tree = tree.adopt( -tr(1)-tr(2) );
    /// let mut iter = tree.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(2) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn adopt( mut self, children: Forest<T> ) -> Self {
        self.0.adopt( children );
        self
    }

    /// Prepend the given tree at the begin of the `Tree`'s children list.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,fr};
    ///
    /// let mut tree = tr(0);
    /// tree = tree.prepend( tr(1) );
    /// tree = tree.prepend( tr(2) );
    /// let mut iter = tree.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(2) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn prepend( mut self, child: Self ) -> Self {
        self.0.push_front( child );
        self
    }

    /// Prepend the given tree at the begin of the `Tree`'s children list.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,fr};
    ///
    /// let mut tree = tr(0);
    /// tree = tree.append( tr(1) );
    /// tree = tree.append( tr(2) );
    /// let mut iter = tree.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(2) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn append( mut self, child: Self ) -> Self {
        self.0.push_back( child );
        self
    }
}

impl<T> Deref for Tree<T> {
    type Target = Node<T>;

    fn deref( &self ) -> &Node<T> { &self.0 }
}

impl<T> DerefMut for Tree<T> {
    fn deref_mut( &mut self ) -> &mut Node<T> { &mut self.0 }
}

impl<T> Forest<T> {
    /// Makes an empty `Forest`
    #[inline]
    pub fn new() -> Forest<T> {
        Forest::<T>{ tail: None, marker: PhantomData }
    }

    /// add the child as the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut forest = tr(1)-tr(2);
    /// forest.push_front( tr(3) );
    /// let mut iter = forest.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(3) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(2) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn push_front( &mut self, tree: Tree<T> ) {
        unsafe { 
            let node = Box::into_raw_non_null( tree.0 );
            match self.tail {
                None       => {
                    (*node.as_ptr()).next_sib = Some(node);
                    self.tail = Some(node);
                }
                Some(tail) => {
                    let tail = tail.as_ptr();
                    (*node.as_ptr()).next_sib = (*tail).next_sib;
                    (*tail).next_sib = Some(node);
                }
            }
        }
    } 

    /// Add the child as the first child and return the `Forest` it`self`.
    #[inline]
    pub fn prepend( mut self, node: Tree<T> ) -> Self {
        self.push_front( node );
        self
    }

    /// Removes and returns the tree at the front of the forest.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,fr};
    ///
    /// let mut forest = tr(0)-tr(1)-tr(2);
    /// let tree = forest.pop_front();
    /// assert_eq!( tree.unwrap(), tr(0) );
    /// assert_eq!( forest, tr(1)-tr(2) );
    /// ```
    #[inline]
    pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        unsafe {
            self.tail.map( |self_tail| {
                let tail = self_tail.as_ptr();
                if (*tail).next_sib == Some(self_tail) {
                    (*tail).next_sib = None;
                    self.tail = None;
                    Tree( Box::from_raw( tail ))
                } else {
                    let head = (*tail).next_sib.unwrap().as_ptr();
                    (*tail).next_sib = (*head).next_sib;
                    (*head).next_sib = None;
                    Tree( Box::from_raw( head ))
                }
            })
        }
    }

    /// Appends the given tree at the end of this forest
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut forest = tr(1)-tr(2);
    /// forest.push_back( tr(3) );
    /// let mut iter = forest.children();
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(1) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(2) );
    /// assert_eq!( iter.next().unwrap().to_owned(), tr(3) );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn push_back( &mut self, child: Tree<T> ) {
        unsafe {
            self.push_front( child );
            if let Some(head) = (*self.tail.unwrap().as_ptr()).next_sib {
                self.tail = Some(head);
            }
        }
    }

    /// Appends the given tree at the end of this forest and return the forest.
    #[inline]
    pub fn append( mut self, child: Tree<T> ) -> Self {
        self.push_back( child );
        self
    }

    /// Merges two forests into one, and return it
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,fr};
    ///
    /// let mut forest = fr().append( tr(1) ).append( tr(2) );
    /// let f2 = fr().append( tr(3) ).append( tr(4) );
    /// forest = forest.merge( f2 );
    /// assert_eq!( forest, tr(1)-tr(2)-tr(3)-tr(4) );
    /// ```
    #[inline]
    pub fn merge( mut self, mut other: Self ) -> Self {
        if let Some(other_tail) = other.tail {
            unsafe {
                match self.tail {
                    None => return other,
                    Some(self_tail) => {
                        let self_tail = self_tail.as_ptr();
                        let other_tail = other_tail.as_ptr();
                        let head = (*self_tail).next_sib;
                        (*self_tail).next_sib = (*other_tail).next_sib;
                        (*other_tail).next_sib = head;
                        self.tail = other.tail;
                        other.tail = None;
                    }
                }
            }
        }
        self
    }

    /// Provides a forward iterator over the `Forest`'s direct decendants
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let forest = tr(1)/tr(2)/tr(3) - tr(4)/tr(5)/tr(6);
    /// let mut iter = forest.children();
    /// //assert_eq!( iter.next().unwrap().clone(), tr(1) /tr(2)/tr(3) );
    /// //assert_eq!( iter.next().unwrap().clone(), tr(4) /tr(5)/tr(6) );
    /// //assert_eq!( iter.next(), None );
    /// ```
    #[inline]
    pub fn children( &self ) -> Iter<T> {
        unsafe {
            match self.tail {
                None => Iter{ head: None, tail: None, marker: PhantomData },
                Some(tail) => Iter {
                    head   : (*tail.as_ptr()).next_sib,
                    tail   : self.tail,
                    marker : PhantomData,
                }
            }
        }
    }

    /// Provides a forward iterator over the `Forest`'s direct decendants with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut forest = (
    ///     - ( tr(1) /tr(2)/tr(3) )
    ///     - ( tr(4) /tr(5)/tr(6) )
    /// );
    /// for mut tree in forest.children_mut() {
    ///     tree.data *= 10;
    /// }
    /// let expected = (
    ///     - ( tr(10) /tr(2)/tr(3) )
    ///     - ( tr(40) /tr(5)/tr(6) )
    /// );
    /// assert_eq!( forest, expected );
    /// ```
    #[inline]
    pub fn children_mut( &mut self ) -> IterMut<T> {
        unsafe {
            match self.tail {
                None => IterMut{ head: None, tail: None, marker: PhantomData },
                Some(tail) => IterMut {
                    head   : (*tail.as_ptr()).next_sib,
                    tail   : Some(tail),
                    marker : PhantomData,
                }
            }
        }
    }

    /// Provides an iterator over the `Forest`'s subtrees for insert/remove at any position.
    ///
    /// # Examples
    ///
    /// ## insert after
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut forest = tr(1)-tr(2)-tr(3);
    /// for mut sub in forest.subtrees() {
    ///     sub.insert_sib( tr(3) );
    /// }
    /// assert_eq!( forest, tr(1)-tr(3)-tr(2)-tr(3)-tr(3)-tr(3) );
    /// ```
    ///
    /// ## insert before
    ///
    /// ```
    /// use trees::tr;
    ///
    /// let mut forest = tr(1)-tr(3)-tr(4);
    /// let mut iter = forest.subtrees().peekable();
    /// while let Some(mut sub) = iter.next() { 
    ///     if let Some(next_sub) = iter.peek() {
    ///         if next_sub.data == 3 {
    ///             sub.insert_sib( tr(2) );
    ///         }
    ///     }
    /// }
    /// assert_eq!( forest, tr(1)-tr(2)-tr(3)-tr(4) );
    /// ```
    ///
    /// ## remove
    /// ```
    /// use trees::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2)/tr(3)/tr(4)/tr(5)/tr(6);
    /// for mut sub in tree.subtrees() {
    ///     let d = sub.data;
    ///     if d%2 == 0 || d%3 == 0 {
    ///         sub.remove();
    ///     }
    /// }
    /// assert_eq!( tree, tr(0) /tr(1)/tr(5) );
    /// ```
    #[inline]
    pub fn subtrees( &mut self ) -> SubtreeIter<T> {
        unsafe {
            match self.tail {
                None => SubtreeIter {
                    next: None, curr: None, prev: None, tail: None,
                    ptail: &mut self.tail as *mut Option<NonNull<Node<T>>>,
                    marker: PhantomData,
                },
                Some(tail) => SubtreeIter {
                    next   : (*tail.as_ptr()).next_sib,
                    curr   : None,
                    prev   : Some(tail),
                    tail   : self.tail,
                    ptail  : &mut self.tail as *mut Option<NonNull<Node<T>>>,
                    marker : PhantomData,
                }
            }
        }
    }

    /// Returns `true` if the `Forest` is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,fr};
    ///
    /// let mut forest = fr();
    /// assert!( forest.is_empty() );
    /// forest.push_back( tr(1) ); 
    /// assert!( !forest.is_empty() );
    /// ```
    #[inline]
    pub fn is_empty( &self ) -> bool { self.tail.is_none() }
}

impl<T> Default for Forest<T> {
    #[inline] fn default() -> Self { fr() }
}

impl<T> Drop for Node<T> {
    fn drop( &mut self ) { self.abandon(); }
}

impl<T> Drop for Forest<T> {
    fn drop( &mut self ) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T:Hash> Hash for Node<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        self.data.hash( state );
        for child in self.children() {
            child.data.hash( state );
        }
    }
}

impl<T:Hash> Hash for Forest<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        for child in self.children() {
            child.hash( state );
        }
    }
}

impl<T:Debug> Debug for Tree<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        self.0.fmt( f )
    }
}

impl<T:Debug> Debug for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            write!( f, "{:?}", self.data )
        } else {
            write!( f, "{:?}", self.data )?;
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{:?} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:Display> Display for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            write!( f, "{}", self.data )
        } else {
            write!( f, "{}", self.data )?;
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:Debug> Debug for Forest<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{:?} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:Display> Display for Forest<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

unsafe impl<T: Send> Send for Node<T> {}
unsafe impl<T: Sync> Sync for Node<T> {}

unsafe impl<T: Send> Send for Forest<T> {}
unsafe impl<T: Sync> Sync for Forest<T> {}

unsafe impl<'a, T: Sync> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}

pub type StrTree   = Tree  <&'static str>;
pub type StrForest = Forest<&'static str>;
pub type StrNode   = Node  <&'static str>;

// - Tree
impl<T> Neg for Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn neg( self ) -> Forest<T> {
        fr().append( self )
    }
}
 
// - &Tree
impl<'a,T:Clone> Neg for &'a Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn neg( self ) -> Forest<T> {
        fr().append( self.clone() )
    }
}

// Tree - Tree
impl<T> Sub<Self> for Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Self ) -> Forest<T> {
        fr().append( self ).append( rhs )
    } 
}

// Tree - &Tree
impl<'a,T:Clone> Sub<&'a Tree<T>> for Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'a Tree<T> ) -> Forest<T> {
        fr().append( self ).append( rhs.clone() )
    } 
}

// &Tree - Tree
impl<'a,T:Clone> Sub<Tree<T>> for &'a Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Tree<T> ) -> Forest<T> {
        fr().append( self.clone() ).append( rhs )
    } 
}

// &Tree - &Tree
impl<'a,T:Clone> Sub<Self> for &'a Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Self ) -> Forest<T> {
        fr().append( self.clone() ).append( rhs.clone() )
    } 
}

// Tree / Forest
impl<T> Div<Forest<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Forest<T> ) -> Tree<T> {
        self.adopt( rhs )
    } 
}

// Tree / &Forest
impl<'a,T:Clone> Div<&'a Forest<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: &'a Forest<T> ) -> Tree<T> {
        self.adopt( rhs.clone() )
    } 
}

// &Tree / Forest
impl<'a,T:Clone> Div<Forest<T>> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Forest<T> ) -> Tree<T> {
        self.clone().adopt( rhs )
    } 
}

// &Tree / &Forest
impl<'a,T:Clone> Div<&'a Forest<T>> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: &'a Forest<T> ) -> Tree<T> {
        self.clone().adopt( rhs.clone() )
    } 
}

// Tree / Tree
impl<T> Div<Tree<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Tree<T> ) -> Tree<T> {
        self.append( rhs )
    } 
}

// Tree / &Tree
impl<'a,T:Clone> Div<&'a Tree<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: &'a Tree<T> ) -> Tree<T> {
        self.append( rhs.clone() )
    } 
}

// &Tree / Tree
impl<'a,T:Clone> Div<Tree<T>> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Tree<T> ) -> Tree<T> {
        self.clone().append( rhs )
    } 
}

// &Tree / &Tree
impl<'a,T:Clone> Div<Self> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Self ) -> Tree<T> {
        self.clone().append( rhs.clone() )
    } 
}

// Tree / ()
impl<T> Div<()> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, _rhs: () ) -> Tree<T> {
        self
    } 
}

// &Tree / ()
impl<'a,T:Clone> Div<()> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, _rhs: () ) -> Tree<T> {
        self.clone()
    } 
}

// Forest - Tree
impl<T> Sub<Tree<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Tree<T> ) -> Self {
        self.append( rhs )
    }
}

// Forest - &Tree
impl<'a,T:Clone> Sub<&'a Tree<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'a Tree<T> ) -> Self {
        self.append( rhs.clone() )
    }
}

// &Forest - Tree
impl<'a,T:Clone> Sub<Tree<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Tree<T> ) -> Forest<T> {
        self.clone().append( rhs )
    }
}

// &Forest - &Tree
impl<'a,'b,T:Clone> Sub<&'b Tree<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'b Tree<T> ) -> Forest<T> {
        self.clone().append( rhs.clone() )
    }
}

// Forest - Forest
impl<T> Sub<Forest<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Self ) -> Self {
        self.merge( rhs )
    }
}

// Forest - &Forest
impl<'a,T:Clone> Sub<&'a Forest<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'a Forest<T> ) -> Self {
        self.merge( rhs.clone() )
    }
}

// &Forest - Forest
impl<'a,T:Clone> Sub<Forest<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Forest<T> ) -> Forest<T> {
        self.clone().merge( rhs )
    }
}

// &Forest - &Forest
impl<'a,'b,T:Clone> Sub<&'b Forest<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'b Forest<T> ) -> Forest<T> {
        self.clone().merge( rhs.clone() )
    }
}

/// Creates a `Tree` with given data on heap.
/// `Tree` is NOT nullable. Consider using an empty `Forest` instead if needed.
///
/// # Examples
///
/// ```
/// use trees::tr;
///
/// let tree = tr(1);
/// assert_eq!( tree.data, 1 );
/// ```
#[inline]
pub fn tr<T>( data: T ) -> Tree<T> {
    Tree::<T>::new( data )
}

/// Makes an empty `Forest`
#[inline]
pub fn fr<T>() -> Forest<T> {
    Forest::<T>::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_drop() {
        struct I( i32, *mut usize );

        fn i( value: i32, pcount: *mut usize ) -> Tree<I> {
            unsafe{ *pcount += 1; }
            tr( I( value, pcount ))
        }

        impl Drop for I {
            fn drop( &mut self ) {
                unsafe { *(self.1) -= 1; }
            }
        }

        impl Debug for I {
            fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
                write!( f, "{}", self.0 )
            }
        }

        let mut count:usize = 0;
        let c = &mut count as *mut usize;
        let tree = i(0,c) /( i(1,c)/i(2,c)/i(3,c) ) /( i(4,c)/i(5,c)/i(6,c) );
        assert_eq!( count, 7 );
        drop( tree );
        assert_eq!( count, 0 );
    }

    #[test]
    fn tree_equal() {
        let tr1 = tr(0) /tr(1)/tr(2);
        let tr2 = tr(0) /tr(1)/tr(2);
        let tr3 = tr(0) /tr(1)/tr(3);
        assert!( tr1 == tr2 );
        assert!( tr1 != tr3 );
    }

    #[test]
    fn tree_expression() {
        let initial =
            tr(0).adopt( fr()
                .append(
                    tr(1).adopt( fr()
                    .append( tr(2) )
                    .append( tr(3) ) ))
                .append(
                    tr(4).adopt( fr()
                    .append( tr(5) )
                    .append( tr(6) ) ))
            );
        let classic =
            tr(0) /(
                -( tr(1) /( -tr(2)-tr(3) ) )
                -( tr(4) /( -tr(5)-tr(6) ) )
            );
        let mordern = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
        assert_eq!( initial, classic );
        assert_eq!( initial, mordern );
    }

    #[test]
    fn forest_from() {
        let trees = [ tr(0), tr(1)/tr(2)/tr(3), tr(4)/tr(5)/tr(6) ];
        let forest = trees.iter().cloned().collect::<Forest<_>>();
        assert_eq!( forest, tr(0) - tr(1)/tr(2)/tr(3) - tr(4)/tr(5)/tr(6) );
    }

    #[test]
    fn forest_into() {
        let forest = tr(0) - tr(1)/tr(2)/tr(3) - tr(4)/tr(5)/tr(6);
        let mut iter = forest.into_iter();
        assert_eq!( iter.next().unwrap(), tr(0) );
        assert_eq!( iter.next().unwrap(), tr(1)/tr(2)/tr(3) );
        assert_eq!( iter.next().unwrap(), tr(4)/tr(5)/tr(6) );
        assert_eq!( iter.next(), None );
    }


    #[test]
    fn tree_extend() {
        let mut tree = tr(0) /tr(1);
        tree.extend( tr(2)-tr(3) );
        assert_eq!( tree, tr(0) /tr(1)/tr(2)/tr(3) );
    }

    #[test]
    fn forest_extend() {
        let mut forest = tr(1)-tr(2);
        forest.extend( tr(3)-tr(4) );
        assert_eq!( forest, tr(1)-tr(2)-tr(3)-tr(4) );
    }
}
