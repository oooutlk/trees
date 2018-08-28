//! A full functional mutable iterator implementation with the extra ability of inserting/removing `Node` at any position than `IterMut`.

use super::{Node,Link,Tree,Size};
use rust::*;

/// Wrapper of `Node` for allowing modification of parent or sib links.
/// Any `Node` that is the root of some `Tree` is impossible to be `Subnode`.
pub struct Subnode<'a, T:'a>{
    node   : &'a mut Node<T>,
    parent : *mut Link,
}

impl<'a, T:'a> Subnode<'a,T> {
    /// Insert sib tree before `self`.
    /// The newly inserted node will not be iterated over by the currently running iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for mut sub in tree.onto_iter() { sub.insert_before( tr(3) ); }
    /// assert_eq!( tree.to_string(), "0( 3 1 3 2 )" );
    /// ```
    #[inline] pub fn insert_before( &mut self, mut sib: Tree<T> ) {
        unsafe {
            (*self.node.prev).next = sib.root_mut().plink();
            sib.root_mut().set_sib( self.node.prev, self.node.plink() );
            self.node.prev = sib.root_mut().plink();
            sib.root_mut().set_parent( self.node.parent );
            (*self.parent).size += Size{ degree: 1, node_cnt: sib.root().size.node_cnt };
        }
        sib.clear();
    }

    /// Insert sib tree after `self`.
    /// The newly inserted node will not be iterated over by the currently running iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for mut sub in tree.onto_iter() { sub.insert_after( tr(3) ); }
    /// assert_eq!( tree.to_string(), "0( 1 3 2 3 )" );
    /// ```
    #[inline] pub fn insert_after( &mut self, mut sib: Tree<T> ) {
        unsafe {
            (*self.node.next).prev = sib.root_mut().plink();
            sib.root_mut().set_sib( self.node.plink(), self.node.next );
            self.node.next = sib.root_mut().plink();
            let parent = self.node.parent;
            sib.root_mut().set_parent( parent );
            (*self.parent).size += Size{ degree: 1, node_cnt: sib.root().size.node_cnt };
            if (*parent).tail() == self.node.plink() {
                (*parent).set_child( sib.root_mut().plink() ); 
            }
        }
        sib.clear();
    }

    /// The subtree departs from its parent and becomes an indepent `Tree`.
    ///
    /// # Examples
    /// ```
    /// use trees::linked::fully::{tr,fr};
    ///
    /// let mut forest = -tr(1)-tr(2)-tr(3);
    /// //for sub in forest.onto_iter() { sub.depart(); }
    /// //forest.onto_iter().next().unwrap().depart();
    /// //assert_eq!( forest, fr() );
    /// ```
    #[inline] pub fn depart( self ) -> Tree<T> {
        unsafe {
            if (*self.parent).tail() == self.node.plink() {
                (*self.parent).set_child( if self.node.has_no_sib() { null_mut() } else { self.node.prev });
            }
            (*self.parent).size -= Size{ degree: 1, node_cnt: self.node.size.node_cnt };
            self.node.reset_parent();
            (*self.node.prev).next = self.node.next;
            (*self.node.next).prev = self.node.prev;
            Tree::from( self.node.plink() )
        }
    }
}

impl<'a, T:'a> Deref for Subnode<'a,T> {
    type Target = Node<T>;
    fn deref( &self ) -> &Node<T> { self.node }
}

impl<'a, T:'a> DerefMut for Subnode<'a,T> { fn deref_mut( &mut self ) -> &mut Node<T> { self.node }}

/// Mutable iterator allowing modification of parent or sib links.
pub struct OntoIter<'a, T:'a>{
    pub(crate) next   : *mut Link,
    pub(crate) curr   : *mut Link,
    pub(crate) prev   : *mut Link,
    pub(crate) child  : *mut Link,
    pub(crate) parent : *mut Link,
    pub(crate) mark   : PhantomData<&'a mut Node<T>>,
}

impl<'a, T:'a> Iterator for OntoIter<'a,T> {
    type Item = Subnode<'a,T>;

    #[inline] fn next( &mut self ) -> Option<Subnode<'a,T>> {
        if !self.child.is_null() {
            if !self.curr.is_null() {
                if self.curr == self.child || self.curr == self.next {
                    return None;
                }
                unsafe { 
                    if (*self.prev).next != self.next { 
                        self.prev = self.curr; // curr is not depart()-ed
                    }
                }
            }
            self.curr = self.next;
            if !self.next.is_null() {
                let curr = self.next;
                unsafe { 
                    self.next = (*curr).next;
                    return Some( Subnode{ node: &mut *( curr as *mut Node<T> ), parent: self.parent });
                }
            }
        }
        None
    }
}

impl<'a, T> ExactSizeIterator for OntoIter<'a, T> {}

impl<'a, T> FusedIterator for OntoIter<'a, T> {}
