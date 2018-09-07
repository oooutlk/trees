//! `Tree` composed of hierarchical `Node`s.

use super::{Node,Link,Forest};
use super::{heap,bfs};
use super::forest::IntoIter;
use rust::*;

/// A non-nullable tree
pub struct Tree<T> {
    pub(crate) root : *mut Node<T>,
               mark : heap::Phantom<T>,
}

impl<T> Tree<T> {
    /// Creates a `Tree` with given data on heap.
    #[inline] pub fn new( data: T ) -> Self { Self::from( heap::make_node( data ) as *mut Link )}

    #[inline] pub fn root( &self ) -> &Node<T> { unsafe { & *self.root }}
    #[inline] pub fn root_mut( &mut self ) -> &mut Node<T> { unsafe { &mut *self.root }}

    #[inline] fn into_data( self ) -> T {
        let data = unsafe{ ptr::read( &self.root().data )};
        self.clear();
        data
    }

    /// Removes and returns the given `Tree`'s children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.abandon().to_string(), "( 1 2 )" );
    /// assert_eq!( tree, tr(0) );
    /// ```
    #[inline] pub fn abandon( &mut self ) -> Forest<T> {
        let forest = Forest::<T>::from( self.root().tail(), self.root().size );
        self.root_mut().reset_child();
        self.size.degree = 0;
        self.size.node_cnt = 1;
        forest
    }

    /// Provides a forward iterator with owned data in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bfs;
    /// use trees::linked::fully::tr;
    ///
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let visits = tree.bfs_into_iter().collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit::Data(0),
    ///     bfs::Visit::GenerationEnd,
    ///     bfs::Visit::Data(1),
    ///     bfs::Visit::Data(4),
    ///     bfs::Visit::GenerationEnd,
    ///     bfs::Visit::Data(2),
    ///     bfs::Visit::Data(3),
    ///     bfs::Visit::SiblingsEnd,
    ///     bfs::Visit::Data(5),
    ///     bfs::Visit::Data(6),
    ///     bfs::Visit::GenerationEnd,
    /// ]);
    /// ```
    pub fn bfs_into_iter( self ) -> bfs::BfsIter<T,IntoIter<T>> { bfs::BfsIter::from( self, 0 )}

    #[inline] pub(crate) fn from( root: *mut Link ) -> Self { Tree{ root: root as *mut Node<T>, mark: PhantomData }}
    #[inline] pub(crate) fn clear( mut self ) { self.root = null_mut(); }
}

impl<T> bfs::Split<T,Self,IntoIter<T>> for Tree<T> {
    fn split( mut self ) -> ( Option<T>, Option<IntoIter<T>> ) {
        let iter = if self.is_leaf() {
            None
        } else {
            Some( self.abandon().into_iter() )
        };
        ( Some( self.into_data() ), iter )
    }
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
