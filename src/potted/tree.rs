use super::{Pot,Forest,NodeRef,NodeMut,MovedNodes,Iter,IterMut,TupleTree,TupleForest,Index};

use super::bfs::{Bfs,BfsTree,Splitted,Visit,Moved};

use rust::*;

#[derive(Debug,PartialEq,Eq)]
pub struct Tree<T> {
    pub(crate) pot : Pot<T>,
}

impl<T> Tree<T> {
    pub fn root( &self ) -> NodeRef<T> {
        NodeRef::new( 1, &self.pot )
    }

    pub fn root_mut( &mut self ) -> NodeMut<T> {
        NodeMut::new( 1, &mut self.pot )
    }

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
        let root_data = unsafe{ ptr::read( self.pot.nodes.as_ptr().offset(1) ).data };
        self.pot.nodes[1].size.node_cnt -= 1;
        let len = self.pot.nodes.len();
        let cap = self.pot.nodes.capacity();
        let nodes = self.pot.nodes.as_mut_ptr(); 
        mem::forget( self );
        let nodes = unsafe{ Vec::from_raw_parts( nodes, len, cap )};
        ( root_data, Forest{ pot: Pot{ nodes }})
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
    pub fn into_bfs<'a>( mut self ) -> BfsTree<MovedNodes<'a,T,Splitted<Iter<'a,T>>>> {
        let root: NodeRef<'a,T> = unsafe{ mem::transmute( self.root() )};
        let bfs = root.bfs();
        let ( iter, size ) = ( MovedNodes::new( Moved( bfs.iter ), &mut self.pot.nodes ), bfs.size );
        mem::forget( self );
        BfsTree{ iter, size }
    }

    pub fn iter<'a, 's:'a>( &'s self ) -> Iter<'a,T> { self.root().iter() }
    pub fn iter_mut<'a, 's:'a>( &'s mut self ) -> IterMut<'a,T> { self.root_mut().iter_mut() }

    pub fn prepend_tr<Tr>( &mut self, tuple: Tr ) where Tr: TupleTree<Data=T> { self.root_mut().prepend_tr( tuple ); }
    pub fn append_tr<Tr>( &mut self, tuple: Tr ) where Tr: TupleTree<Data=T> { self.root_mut().append_tr( tuple ); }
    pub fn insert_tr<Tuple>( &mut self, nth: usize, tuple: Tuple ) where Tuple: TupleTree<Data=T> { self.root_mut().insert_tr( nth, tuple ); }

    pub fn prepend_fr<Fr>( &mut self, tuple: Fr ) where Fr: TupleForest<Data=T> { self.root_mut().prepend_fr( tuple ); }
    pub fn append_fr<Fr>( &mut self, tuple: Fr ) where Fr: TupleForest<Data=T> { self.root_mut().append_fr( tuple ); }

    pub fn drop_front( &mut self ) { self.root_mut().drop_front(); }
    pub fn drop_back( &mut self ) { self.root_mut().drop_back(); }
    pub fn drop_nth( &mut self, nth: usize ) { self.root_mut().drop_nth( nth ); }

    pub fn prepend_bfs<Iter>( &mut self, bfs: Bfs<Iter> ) where Iter : Iterator<Item=Visit<T>> { self.root_mut().prepend_bfs( bfs ); }
    pub fn append_bfs<Iter>( &mut self, bfs: Bfs<Iter> ) where Iter : Iterator<Item=Visit<T>> { self.root_mut().append_bfs( bfs ); }
}

impl<T,Tuple> From<Tuple> for Tree<T>
    where Tuple: TupleTree<Data=T>
{
    fn from( tuple: Tuple ) -> Self {
        let mut pot = Pot::new();
        tuple.construct_all_nodes( usize::null(), &mut pot );
        mem::forget( tuple );
        Tree{ pot }
    }
}

impl<T,Iter> From<BfsTree<Iter>> for Tree<T>
    where Iter : Iterator<Item=Visit<T>>
{
    fn from( tree_iter: BfsTree<Iter> ) -> Self {
        let mut pot = Pot::new();
        let dummy_mut = NodeMut::new( usize::null(), &mut pot );
        dummy_mut.append_bfs( tree_iter.wrap() );
        Tree{ pot }
    }
}

impl<T> Drop for Tree<T> {
    fn drop( &mut self ) {
        if !self.pot.nodes.is_empty() {
            self.root_mut().drop_all_data_if_needed();
            unsafe{ self.pot.nodes.set_len(0); }
        }
    }
}
impl<T:Display> Display for Tree<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result { self.root().fmt(f) }
}
 
#[cfg(test)]
mod tests {
    use super::*;
    use linked::fully::tr;

    #[test] fn from_tuple() {
        let tuple = ( 0, (1,2,3), (4,5,6) );
        let potted = Tree::<i32>::from( tuple );
        assert_eq!( potted.root().to_string(), "0( 1( 2 3 ) 4( 5 6 ) )" );
    }

    #[test] fn from_bfs() {
        let linked = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
        let potted = Tree::<i32>::from( linked.into_bfs() );
        assert_eq!( potted.root().to_string(), "0( 1( 2 3 ) 4( 5 6 ) )" );
    }
}
