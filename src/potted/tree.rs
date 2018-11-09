//! `Tree` composed of hierarchical `Node`s.

use super::{Pot,Node,MovedNodes,Forest,Iter,TupleTree,NullIndex,ROOT,NULL};

use super::bfs::{BfsTree,Splitted,Visit,Moved};

use rust::*;

pub struct Tree<T> {
    pub(crate) pot : Pot<T>,
}

impl<T> Tree<T> {
    pub fn root( &self ) -> &Node<T> { &self.pot[ ROOT ]}
    pub fn root_mut( &mut self ) -> &mut Node<T> { &mut self.pot[ ROOT ]}

    /// Break the tree into root's data and the children forest.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::Tree;
    ///
    /// let tree: Tree<_> = ( 1, (2,3,4), (5,6,7) ).into();
    /// let ( root_data, forest ) = tree.abandon();
    /// assert_eq!( root_data, 1 );
    /// assert_eq!( forest.to_string(), "( 2( 3 4 ) 5( 6 7 ) )" );
    /// ```
    pub fn abandon( mut self ) -> ( T, Forest<T> ) {
        self.root_mut().size.node_cnt -= 1;
        let root_data = unsafe{ ptr::read( self.root() ).data };
        let mut pot = self.pot;
        pot.set_forest_pot();
        forget( self );
        ( root_data, Forest{ pot })
    }

    /// For debug purpose.
    pub fn pot( &self ) -> &Pot<T> { &self.pot }

    /// Provides a forward iterator with owned data in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::potted::Tree;
    ///
    /// let tree: Tree<_> = (0, (1,2,3), (4,5,6) ).into();
    /// let visits = tree.into_bfs().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: 0, size: Size{ degree: 2, node_cnt: 7 }},
    ///     bfs::Visit{ data: 1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: 4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: 2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn into_bfs<'a>( self ) -> BfsTree<MovedNodes<'a,T,Splitted<Iter<'a,T>>>> {
        let root: &'a Node<T> = unsafe{ transmute( self.root() )};
        let bfs = root.bfs();
        let ( iter, size ) = ( MovedNodes::new( Moved( bfs.iter ), self.pot ), bfs.size );
        forget( self );
        BfsTree{ iter, size }
    }
}

impl<T> Borrow<Node<T>> for Tree<T> { fn borrow( &self ) -> &Node<T> { self.root() }}
impl<T> BorrowMut<Node<T>> for Tree<T> { fn borrow_mut( &mut self ) -> &mut Node<T> { self.root_mut() }}

impl<T> Deref for Tree<T> {
    type Target = Node<T>;
    fn deref( &self ) -> &Node<T> { &*self.root() }
}

impl<T> DerefMut for Tree<T> {
    fn deref_mut( &mut self ) -> &mut Node<T> { &mut *self.root_mut() }
}

impl<T,Tuple> From<Tuple> for Tree<T>
    where Tuple: TupleTree<Data=T>
{
    fn from( tuple: Tuple ) -> Self {
        let pot = Pot::new_tree();
        tuple.construct_all_nodes( usize::null(), pot );
        forget( tuple );
        Tree{ pot }
    }
}

impl<T,Iter> From<BfsTree<Iter>> for Tree<T>
    where Iter : Iterator<Item=Visit<T>>
{
    fn from( tree_iter: BfsTree<Iter> ) -> Self {
        let mut pot = Pot::new_tree();
        pot[ NULL ].append_bfs( tree_iter.wrap() );
        Tree{ pot }
    }
}

impl<T> Drop for Tree<T> {
    fn drop( &mut self ) {
        if self.pot.new_index() != 0 {
            self.root_mut().drop_all_data_if_needed();
        }
        unsafe{ Pot::drop( self.pot ); }
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

#[cfg(test)]
mod tests {
    use super::*;
    use linked::fully::tr;

    #[test] fn from_tuple() {
        let tuple = ( 0, (1,2,3), (4,5,6) );
        let potted = Tree::<i32>::from( tuple );
        assert_eq!( potted.to_string(), "0( 1( 2 3 ) 4( 5 6 ) )" );
    }

    #[test] fn from_bfs() {
        let linked = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
        let potted = Tree::<i32>::from( linked.into_bfs() );
        assert_eq!( potted.to_string(), "0( 1( 2 3 ) 4( 5 6 ) )" );
    }
}
