//! `Tree` composed of hierarchical `Node`s.

use super::{Node,Forest};
use super::heap;
use rust::*;

/// A non-nullable tree
pub struct Tree<T> {
    pub(crate) root : *mut Node<T>,
               mark : heap::Phantom<T>,
}

impl<T> Tree<T> {
    /// Creates a `Tree` with given data on heap.
    #[inline] pub fn new( data: T ) -> Self { Self::from( heap::make_node( data )) }

    #[inline] pub fn root( &self ) -> &Node<T> { unsafe { & *self.root }}
    #[inline] pub fn root_mut( &mut self ) -> &mut Node<T> { unsafe { &mut *self.root }}

    /// Removes and returns the given `Tree`'s children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.abandon().to_string(), "( 1 2 )" );
    /// assert_eq!( tree, tr(0) );
    /// ```
    #[inline] pub fn abandon( &mut self ) -> Forest<T> {
        let forest = Forest::<T>::from( self.root().tail() );
        self.reset_child();
        forest
    }

    #[inline] pub(crate) fn from( node: *mut Node<T> ) -> Self { Tree{ root: node, mark: PhantomData } }
    #[inline] pub(crate) fn clear( mut self ) { self.root = null_mut(); }
}

impl<T> Borrow<Node<T>> for Tree<T> { fn borrow( &self ) -> &Node<T> { self.root() }}
impl<T> BorrowMut<Node<T>> for Tree<T> { fn borrow_mut( &mut self ) -> &mut Node<T> { self.root_mut() }}

impl<T> Deref for Tree<T> {
    type Target = Node<T>;
    fn deref( &self ) -> &Node<T> { unsafe { & *self.root }}
}

impl<T> DerefMut for Tree<T> {
    fn deref_mut( &mut self ) -> &mut Node<T> { unsafe { &mut *self.root }}
}

impl<T:Clone> Clone for Tree<T> { fn clone( &self ) -> Self { self.root().to_owned() }}

impl<T> Drop for Tree<T> {
    fn drop( &mut self ) {
        if !self.root.is_null() {
            while let Some(_) = self.pop_front() {}
            heap::drop_node( self.root );
        }
    }
}

impl<T:Debug> Debug for Tree<T> { fn fmt( &self, f: &mut Formatter ) -> fmt::Result { write!( f, "{:?}", self.root() )}}

impl<T:Display> Display for Tree<T> { fn fmt( &self, f: &mut Formatter ) -> fmt::Result { write!( f, "{}", self.root() )}}

impl<T:PartialEq> PartialEq for Tree<T> {
    fn eq( &self, other: &Self ) -> bool { self.root().eq( other.root() )}
    fn ne( &self, other: &Self ) -> bool { self.root().ne( other.root() )}
}

impl<T:Eq> Eq for Tree<T> {}

impl<T:PartialOrd> PartialOrd for Tree<T> { #[inline] fn partial_cmp( &self, other: &Self ) -> Option<Ordering> { self.root().partial_cmp( other.root() )}}

impl<T:Ord> Ord for Tree<T> { #[inline] fn cmp( &self, other: &Self ) -> Ordering { self.root().cmp( other.root() )}}

impl<T:Hash> Hash for Tree<T> { fn hash<H:Hasher>( &self, state: &mut H ) { self.root().hash( state )}}

unsafe impl<T:Send> Send for Tree<T> {}
unsafe impl<T:Sync> Sync for Tree<T> {}
