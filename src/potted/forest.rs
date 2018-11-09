//! `Forest` composed of disjoint `Tree`s.

use super::{Pot,Tree,Node,MovedNodes,Iter,IterMut,TupleTree,TupleForest,Size,NullIndex,ROOT,fake_root};

use super::bfs::{Bfs,BfsForest,Splitted,Moved,Visit};

use rust::*;

pub struct Forest<T> {
    pub(crate) pot : Pot<T>,
}

impl<T> Forest<T> {
    fn node( &self, index: usize ) -> &Node<T> { &self.pot[ index ]}
    fn node_mut( &mut self, index: usize ) -> &mut Node<T> { &mut self.pot[ index ]}

    /// Makes an empty `Forest`.
    pub fn new() -> Self {
        let mut pot = Pot::new_forest();
        pot.push( fake_root() );
        Forest{ pot }
    }

    /// Joins the root data in the forest to make a tree.
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
        self.node_mut( ROOT ).size.node_cnt += 1;
        unsafe{ ptr::write( &mut self.node_mut( ROOT ).data, root_data ); }
        let mut pot = self.pot;
        pot.set_tree_pot();
        forget( self );
        Tree{ pot }
    }

    /// For debug purpose.
    pub fn pot( &self ) -> &Pot<T> { &self.pot }

    /// Returns the number of child nodes in `Forest`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TupleForest,TreeData,fr};
    /// let forest = Forest::from(( fr(), 1, 2, 3 ));
    /// assert_eq!( forest.degree(), 3 );
    /// ```
    pub fn degree( &self ) -> usize { self.pot.degree( ROOT )}

    /// Returns `true` if this forest has no child nodes, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TupleForest,TreeData,fr};
    ///
    /// let forest = Forest::<i32>::new();
    /// assert!( forest.is_empty() );
    ///
    /// let forest = Forest::from(( fr(), 1, 2, 3 ));
    /// assert!( !forest.is_empty() );
    /// ```
    pub fn is_empty( &self ) -> bool { self.pot.is_leaf( ROOT )}

    /// Provides a forward iterator over child nodes
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let forest: Forest<_> = ( fr(), 1, 2, 3 ).into();
    /// let mut iter = forest.iter();
    /// assert_eq!( iter.next().unwrap().data, 1 );
    /// assert_eq!( iter.next().unwrap().data, 2 );
    /// assert_eq!( iter.next().unwrap().data, 3 );
    /// assert_eq!( iter.next(), None );
    /// assert_eq!( iter.next(), None );
    /// ```
    pub fn iter( &self ) -> Iter<T> {
        if self.is_empty() {
            Iter::new( usize::null(), 0, self.pot )
        } else {
            Iter::new( self.pot.head( ROOT ), self.pot.degree( ROOT ), self.pot )
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
    ///     child.data *= 10;
    /// }
    /// assert_eq!( forest.to_string(), "( 10 20 30 )" );
    /// ```
    pub fn iter_mut( &mut self ) -> IterMut<T> {
        if self.is_empty() {
            IterMut::new( usize::null(), 0, self.pot )
        } else {
            IterMut::new( self.pot.head( ROOT ), self.pot.degree( ROOT ), self.pot )
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
        let size = self.node( ROOT ).size;
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
        let size = self.node( ROOT ).size;
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
    pub fn into_bfs<'a>( self ) -> BfsForest<MovedNodes<'a,T,Splitted<Iter<'a,T>>>> {
        let bfs: BfsForest<Splitted<Iter<'a,T>>> = unsafe{ mem::transmute( self.bfs() )};
        let ( iter, size ) = ( MovedNodes::new( Moved( bfs.iter ), self.pot ), bfs.size );
        mem::forget( self );
        BfsForest{ iter, size }
    }

    fn fake_root( &mut self ) -> &mut Node<T> { self.node_mut( ROOT )}

    /// Returns the immutable reference of n-th child node if exists, otherwise `None`.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let forest: Forest<_> = ( fr(), "a", "b", "c", "d", "e" ).into();
    /// assert_eq!( forest.nth_child(2).unwrap().data, "c" );
    /// ```
    pub fn nth_child( &self, n: usize ) -> Option<&Node<T>> {
        let pot = self.pot;
        pot.nth_child( ROOT, n ).map( |index| unsafe{ transmute( &pot[ index ])})
    }

    /// Returns the mutable reference of n-th child node if exists, otherwise `None`.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), "a", "b", "c", "d", "e" ).into();
    /// forest.nth_child_mut(2).unwrap().data = "C";
    /// assert_eq!( forest.to_string(), "( a b C d e )" );
    /// ```
    pub fn nth_child_mut( &mut self, n: usize ) -> Option<&mut Node<T>> {
        let mut pot = self.pot;
        pot.nth_child( ROOT, n ).map( |index| unsafe{ transmute( &mut pot[ index ])})
    }

    /// Add the tuple tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleTree,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), "a", "b", "c" ).into();
    /// forest.prepend_tr(( "d", "e", "f" ));
    /// assert_eq!( forest.to_string(), "( d( e f ) a b c )" );
    /// ```
    pub fn prepend_tr<Tr>( &mut self, tuple: Tr ) where Tr: TupleTree<Data=T> { self.fake_root().prepend_tr( tuple ); }

    /// Add the tuple tree as the last child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleTree,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), "a", "b", "c" ).into();
    /// forest.append_tr(( "d", "e", "f" ));
    /// assert_eq!( forest.to_string(), "( a b c d( e f ) )" );
    /// ```
    pub fn append_tr<Tr>( &mut self, tuple: Tr ) where Tr: TupleTree<Data=T> { self.fake_root().append_tr( tuple ); }

    /// Insert the tuple tree as the n-th child.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleTree,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), "a", "b", "c", "d", "e" ).into();
    /// forest.insert_tr( 2, ("f",) );
    /// assert_eq!( forest.to_string(), "( a b f c d e )" );
    /// ```
    pub fn insert_tr<Tuple>( &mut self, nth: usize, tuple: Tuple )
        where Tuple: TupleTree<Data=T>
    {
        self.fake_root().insert_tr( nth, tuple );
    }

    /// Add the tuple forest as the first-n children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), "a", "b", "c" ).into();
    /// forest.prepend_fr(( fr(), "d", "e", "f" ));
    /// assert_eq!( forest.to_string(), "( d e f a b c )" );
    /// ```
    pub fn prepend_fr<Fr>( &mut self, tuple: Fr )
        where Fr: TupleForest<Data=T>
    {
        self.fake_root().prepend_fr( tuple );
    }

    /// Add the tuple forest as the last-n children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), "a", "b", "c" ).into();
    /// forest.append_fr(( fr(), "d", "e", "f" ));
    /// assert_eq!( forest.to_string(), "( a b c d e f )" );
    /// ```
    pub fn append_fr<Fr>( &mut self, tuple: Fr )
        where Fr: TupleForest<Data=T>
    {
        self.fake_root().append_fr( tuple );
    }

    /// Drop the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), 1, 2, 3, 4 ).into();
    /// forest.drop_front();
    /// assert_eq!( forest.to_string(), "( 2 3 4 )" );
    /// ```
    pub fn drop_front( &mut self ) { self.fake_root().drop_front(); }

    /// Drop the last child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), 1, 2, 3, 4 ).into();
    /// forest.drop_back();
    /// assert_eq!( forest.to_string(), "( 1 2 3 )" );
    /// ```
    pub fn drop_back( &mut self ) { self.fake_root().drop_back(); }

    /// Drop the n-th child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut forest: Forest<_> = ( fr(), "a", "b", "c", "d", "e" ).into();
    /// forest.drop_nth( 2 );
    /// assert_eq!( forest.to_string(), "( a b d e )" );
    /// ```
    pub fn drop_nth( &mut self, nth: usize ) { self.fake_root().drop_nth( nth ); }

    /// Add tree(s) from a bfs iterator as the first child(ren).
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut potted: Forest<_> = ( fr(), (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) /tr(6) /tr(7);
    /// potted.nth_child_mut(0).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 5( 6 7 ) 2 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) /t(9) /t(10);
    /// potted.nth_child_mut(1).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 5( 6 7 ) 2 ) 3( 8( 9 10 ) 4 ) )" );
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut potted: Forest<_> = ( fr(), (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) -tr(6) -tr(7);
    /// potted.nth_child_mut(0).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 5 6 7 2 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) -t(9) -t(10);
    /// potted.nth_child_mut(1).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 5 6 7 2 ) 3( 8 9 10 4 ) )" );
    /// ```
    pub fn prepend_bfs<Iter>( &mut self, bfs: Bfs<Iter> )
        where Iter : Iterator<Item=Visit<T>>
    {
        self.fake_root().prepend_bfs( bfs );
    }

    /// Add tree(s) from a bfs iterator as the last child(ren).
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut potted: Forest<_> = ( fr(), (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) /tr(6) /tr(7);
    /// potted.nth_child_mut(0).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 5( 6 7 ) 2 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) /t(9) /t(10);
    /// potted.nth_child_mut(1).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 5( 6 7 ) 2 ) 3( 8( 9 10 ) 4 ) )" );
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Forest,TreeData,TupleForest,fr};
    ///
    /// let mut potted: Forest<_> = ( fr(), (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) -tr(6) -tr(7);
    /// potted.nth_child_mut(0).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 2 5 6 7 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) -t(9) -t(10);
    /// potted.nth_child_mut(1).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.to_string(), "( 1( 2 5 6 7 ) 3( 4 8 9 10 ) )" );
    /// ```
    pub fn append_bfs<Iter>( &mut self, bfs: Bfs<Iter> )
        where Iter : Iterator<Item=Visit<T>>
    {
        self.fake_root().append_bfs( bfs );
    }
}

impl<T,Tuple> From<Tuple> for Forest<T>
    where Tuple : TupleForest<Data=T>
{
    fn from( tuple: Tuple ) -> Self {
        let mut forest = Forest::new();
        tuple.construct_all_nodes( ROOT, forest.pot );
        let degree = tuple.descendants(0) as u32;
        let node_cnt = tuple.nodes() as u32;
        {
            let fake_root = forest.node_mut( ROOT );
            fake_root.size = Size{ degree, node_cnt };
            fake_root.adjoined = degree;
        }
        mem::forget( tuple );
        forest
    }
}

impl<T,Iter> From<BfsForest<Iter>> for Forest<T>
    where Iter : Iterator<Item=Visit<T>>
{
    fn from( forest_iter: BfsForest<Iter> ) -> Self {
        let mut forest = Forest::new();
        forest.append_bfs( forest_iter.wrap() );
        forest
    }
}

impl<T> Default for Forest<T> { #[inline] fn default() -> Self { Self::new() }}

impl<T> Drop for Forest<T> {
    fn drop( &mut self ) {
        if !self.pot.is_empty() {
            self.iter_mut().for_each( |node| node.drop_all_data_if_needed() );
        }
        unsafe{ Pot::drop( self.pot ); }
    }
}

impl<T:Debug> Debug for Forest<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.iter() {
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
            for child in self.iter() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:PartialEq> PartialEq for Forest<T> {
    fn eq( &self, other: &Self ) -> bool { self.iter().eq( other.iter() )}
    fn ne( &self, other: &Self ) -> bool { self.iter().ne( other.iter() )}
}

impl<T:Eq> Eq for Forest<T> {}

impl<T:PartialOrd> PartialOrd for Forest<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        self.iter().partial_cmp( other.iter() )
    }
}

impl<T:Ord> Ord for Forest<T> {
    #[inline] fn cmp( &self, other: &Self ) -> Ordering {
        self.iter().cmp( other.iter() )
    }
}

impl<T:Hash> Hash for Forest<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        for child in self.iter() {
            child.hash( state );
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
        let bfs = linked.into_bfs();
        let potted = Forest::<i32>::from( bfs );
        assert_eq!( potted.to_string(), "( 1( 2 3 ) 4( 5 6 ) )" );
    }
}
