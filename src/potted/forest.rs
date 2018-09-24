use super::{Pot,Tree,Node,NodeMut,MovedNodes,Iter,IterMut,TupleTree,TupleForest,Size,Index};

use super::bfs::{Bfs,BfsForest,Splitted,Moved,Visit};

use rust::*;

pub struct Forest<T> {
    pub(crate) pot : Pot<T>,
}

impl<T> Forest<T> {
    pub fn new() -> Self {
        let mut pot = Pot::new();
        let fake_root = Node {
            next     : 1,
            child    : u32::null(),
            prev     : 1,
            parent   : 0,
            size     : Size{ degree: 0, node_cnt: 0 },
            adjoined : 0,
            data     : unsafe{ mem::uninitialized() },
        };
        pot.nodes.push( fake_root ); // [1] for fake root
        Forest{ pot }
    }

    /// Join the root data in the forest to make a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,fr};
    ///
    /// let forest: Forest<_> = ( fr(), (2,3,4), (5,6,7) ).into();
    /// let tree = forest.adopt( 1 );
    /// assert_eq!( tree.root().to_string(), "1( 2( 3 4 ) 5( 6 7 ) )" );
    /// ```
    pub fn adopt( mut self, root_data: T ) -> Tree<T> {
        self.pot.nodes[1].size.node_cnt += 1;
        unsafe{ ptr::write( &mut self.pot.nodes[1].data, root_data ); }
        let len = self.pot.nodes.len();
        let cap = self.pot.nodes.capacity();
        let nodes = self.pot.nodes.as_mut_ptr(); 
        mem::forget( self );
        let nodes = unsafe{ Vec::from_raw_parts( nodes, len, cap )};
        Tree{ pot: Pot{ nodes }}
    }

    /// For debug purpose.
    pub fn pot( &self ) -> &Pot<T> { &self.pot }

    pub fn degree( &self ) -> usize { self.pot().degree(1) }

    pub fn is_empty( &self ) -> bool { self.pot().is_leaf(1) }

    /// Provides a forward iterator over child nodes
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), 1, 2, 3 ).into();
    /// let mut iter = forest.iter();
    /// assert_eq!( iter.next().unwrap().data(), &1 );
    /// assert_eq!( iter.next().unwrap().data(), &2 );
    /// assert_eq!( iter.next().unwrap().data(), &3 );
    /// assert_eq!( iter.next(), None );
    /// assert_eq!( iter.next(), None );
    /// ```
    pub fn iter( &self ) -> Iter<T> {
        if self.is_empty() {
            Iter::new( usize::null(), 0, &self.pot )
        } else {
            Iter::new( self.pot().head(1), self.pot().degree(1), &self.pot )
        }
    }

    /// Provides a forward iterator over child nodes with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), 1, 2, 3 ).into();
    /// for child in forest.iter_mut() {
    ///     child.set_data( (*child.node_ref().data()) * 10 );
    /// }
    /// assert_eq!( forest.to_string(), "( 10 20 30 )" );
    /// ```
    pub fn iter_mut( &mut self ) -> IterMut<T> {
        if self.is_empty() {
            IterMut::new( usize::null(), 0, &mut self.pot )
        } else {
            IterMut::new( self.pot().head(1), self.pot().degree(1), &mut self.pot )
        }
    }

    /// Provides a forward iterator in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::potted::{Forest,fr};
    ///
    /// let forest: Forest<_> = ( fr(), (1,2,3), (4,5,6) ).into();
    /// let visits = forest.bfs().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: &1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn bfs<'a, 's:'a>( &'s self ) -> BfsForest<Splitted<Iter<'a,T>>> {
        let size = self.pot.nodes[1].size;
        let mut iters = VecDeque::new();
        iters.push_back( self.iter() );
        let iter = Splitted{ iters };
        BfsForest{ iter, size }
    }

    /// Provides a forward iterator with mutable references in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::potted::{Forest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), (1,2,3), (4,5,6) ).into();
    /// let visits = forest.bfs_mut().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: &mut 1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &mut 4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &mut 2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn bfs_mut<'a, 's:'a>( &'s mut self ) -> BfsForest<Splitted<IterMut<'a,T>>> {
        let size = self.pot.nodes[1].size;
        let mut iters = VecDeque::new();
        iters.push_back( self.iter_mut() );
        let iter = Splitted{ iters };
        BfsForest{ iter, size }
    }

    /// Provides a forward iterator with owned data in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::potted::{Forest,fr};
    ///
    /// let mut forest: Forest<_> = (fr(), (1,2,3), (4,5,6) ).into();
    /// let visits = forest.into_bfs().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: 1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: 4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: 2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn into_bfs<'a>( mut self ) -> BfsForest<MovedNodes<'a,T,Splitted<Iter<'a,T>>>> {
        let bfs: BfsForest<Splitted<Iter<'a,T>>> = unsafe{ mem::transmute( self.bfs() )};
        let ( iter, size ) = ( MovedNodes::new( Moved( bfs.iter ), &mut self.pot.nodes ), bfs.size );
        mem::forget( self );
        BfsForest{ iter, size }
    }

    fn fake_root( &mut self ) -> NodeMut<T> { NodeMut::new( 1, &mut self.pot )}

    pub fn prepend_tr<Tr>( &mut self, tuple: Tr ) where Tr: TupleTree<Data=T> { self.fake_root().prepend_tr( tuple ); }
    pub fn append_tr<Tr>( &mut self, tuple: Tr ) where Tr: TupleTree<Data=T> { self.fake_root().append_tr( tuple ); }
    pub fn insert_tr<Tuple>( &mut self, nth: usize, tuple: Tuple ) where Tuple: TupleTree<Data=T> { self.fake_root().insert_tr( nth, tuple ); }

    pub fn prepend_fr<Fr>( &mut self, tuple: Fr ) where Fr: TupleForest<Data=T> { self.fake_root().prepend_fr( tuple ); }
    pub fn append_fr<Fr>( &mut self, tuple: Fr ) where Fr: TupleForest<Data=T> { self.fake_root().append_fr( tuple ); }

    pub fn drop_front( &mut self ) { self.fake_root().drop_front(); }
    pub fn drop_back( &mut self ) { self.fake_root().drop_back(); }
    pub fn drop_nth( &mut self, nth: usize ) { self.fake_root().drop_nth( nth ); }

    pub fn prepend_bfs<Iter>( &mut self, bfs: Bfs<Iter> ) where Iter : Iterator<Item=Visit<T>> { self.fake_root().prepend_bfs( bfs ); }
    pub fn append_bfs<Iter>( &mut self, bfs: Bfs<Iter> ) where Iter : Iterator<Item=Visit<T>> { self.fake_root().append_bfs( bfs ); }
}

impl<T,Tuple> From<Tuple> for Forest<T>
    where Tuple : TupleForest<Data=T>
{
    fn from( tuple: Tuple ) -> Self {
        let mut forest = Forest::new();
        tuple.construct_all_nodes( 1, &mut forest.pot );
        let degree = tuple.descendants(0) as u32;
        let node_cnt = tuple.nodes() as u32;
        forest.pot.nodes[1].size = Size{ degree, node_cnt };
        mem::forget( tuple );
        forest
    }
}

impl<T,Iter> From<BfsForest<Iter>> for Forest<T>
    where Iter : Iterator<Item=Visit<T>>
{
    fn from( forest_iter: BfsForest<Iter> ) -> Self {
        let mut forest = Forest::new();
        let fake_root = NodeMut::new( 1, &mut forest.pot );
        fake_root.append_bfs( forest_iter.wrap() );
        forest
    }
}

impl<T> Drop for Forest<T> {
    fn drop( &mut self ) {
        if !self.pot.nodes.is_empty() {
            self.iter_mut().for_each( |node| node.drop_all_data_if_needed() );
        }
    }
}

impl<T:Display> Display for Forest<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.iter() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linked::fully::tr;
    use super::super::fr;

    #[test] fn from_tuple() {
        let tuple = ( fr(), (2,3,4), (5,6,7) );
        let potted = Forest::<i32>::from( tuple );
        assert_eq!( potted.to_string(), "( 2( 3 4 ) 5( 6 7 ) )" );
    }

    #[test] fn from_bfs() {
        let linked = -( tr(1)/tr(2)/tr(3) ) -( tr(4)/tr(5)/tr(6) );
        let potted = Forest::<i32>::from( linked.into_bfs() );
        assert_eq!( potted.to_string(), "( 1( 2 3 ) 4( 5 6 ) )" );
    }
}
