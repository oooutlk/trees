//! Tree node implementation.

use super::{Pot,Size,Iter,IterMut,TupleTree,TupleForest,NULL,FOREST};
use super::bfs::{Bfs,BfsTree,Splitted,Split,Moved,Visit};

use indexed::Indexed;

use rust::*;

pub trait NullIndex {
    fn null() -> Self;
    fn is_null( self ) -> bool;
}

impl NullIndex for u32 {
    fn null() -> Self { 0 }
    fn is_null( self ) -> bool { self == 0 }
}

impl NullIndex for usize {
    fn null() -> Self { 0 }
    fn is_null( self ) -> bool { self == 0 }
}

pub struct Node<T> {
    pub(crate) next     : u32,  // next sibling
    pub(crate) child    : u32,  // last child
    pub(crate) prev     : u32,  // previous sibling
    pub(crate) parent   : u32,  // parent node
    pub(crate) size     : Size, // count of children and count of all nodes, including itself and all its descendants
    pub(crate) adjoined : u32,  // count of adjioned children.
                                // `adjoined == FOREST` means "this node has no data, it is a forest of which children are all adjoined"
    pub(crate) index    : u32,  // position in the pot
    pub        data     : T,
}

unsafe impl<T> Indexed for Node<T> {
    fn null() -> usize { NULL }
    unsafe fn get_index( &self ) -> usize { self.index as usize }
    unsafe fn set_index( &mut self, index: usize ) { self.index = index as u32; }
}

impl<T> Node<T> {
    #[inline] pub(crate) fn next(   &self ) -> usize { self.next   as usize }
  //pub(crate) fn prev(   &self ) -> usize { self.prev   as usize }
    #[inline] pub(crate) fn child(  &self ) -> usize { self.child  as usize }
    //pub(crate) fn parent( &self ) -> usize { self.parent as usize }
    #[inline] pub(crate) fn index(  &self ) -> usize { self.index  as usize }

    #[inline] pub(crate) fn degree( &self ) -> usize { self.size.degree as usize }
    #[inline] pub(crate) fn adjoined( &self )-> usize { self.adjoined as usize }

    #[inline] pub(crate) fn set_next(   &mut self, index: usize ) { self.next   = index as u32; }
    #[inline] pub(crate) fn set_prev(   &mut self, index: usize ) { self.prev   = index as u32; }
    #[inline] pub(crate) fn set_child(  &mut self, index: usize ) { self.child  = index as u32; }
    #[inline] pub(crate) fn set_parent( &mut self, index: usize ) { self.parent = index as u32; }

    #[inline] pub(crate) fn reset_child( &mut self ) { self.set_child( usize::null() ); }

    #[inline] pub(crate) fn pot( &self ) -> Pot<T> { Pot{ nodes: self.pool_non_null() }}

    #[inline] pub(crate) fn is_forest( &self ) -> bool { self.adjoined == FOREST }

    /// Returns `true` if this node has no child nodes, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,Node,TreeData,TupleTree};
    ///
    /// let mut tree = Tree::<_>::from(( 1, ));
    /// let root: &mut Node<_> = tree.root_mut();
    /// assert!( root.is_leaf() );
    ///
    /// root.append_tr(( 2, ));
    /// assert!( !root.is_leaf() );
    /// ```
    #[inline] pub fn is_leaf( &self ) -> bool { self.child.is_null() }

    #[deprecated( since="0.2.1", note="please use `data` field instead" )]
    #[inline] pub fn data( &self ) -> &T { &self.data }

    #[deprecated( since="0.2.1", note="please use `data` field instead" )]
    #[inline] pub fn data_mut( &mut self ) -> &mut T { &mut self.data }

    #[deprecated( since="0.2.1", note="please use `data` field instead" )]
    #[inline] pub fn set_data( &mut self, data: T ) { self.data = data; }

    /// Returns reference of parent node if exists, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,Node,TreeData,TupleTree};
    ///
    /// let mut tree = Tree::<_>::from(( 1, 2 ));
    /// let root  : &Node<_> = tree.root();
    /// let child : &Node<_> = root.iter().next().unwrap();
    /// assert_eq!( root.parent(),  None );
    /// assert_eq!( child.parent(), Some( root ));
    /// ```
    #[inline] pub fn parent( &self ) -> Option<&Self> {
        if self.parent.is_null() {
            None
        } else {
            Some( unsafe{ transmute( &self.pot()[ self.parent as usize ])})
        }
    }

    /// Returns the immutable reference of n-th child node if exists, otherwise `None`.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// assert_eq!( tree.root().nth_child(2).unwrap().data, "d" );
    /// ```
    pub fn nth_child( &self, n: usize ) -> Option<&Self> {
        let pot = self.pot();
        pot.nth_child( self.index(), n ).map( |index| unsafe{ transmute( &pot[ index ])})
    }

    /// Returns the mutable reference of n-th child node if exists, otherwise `None`.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// tree.root_mut().nth_child_mut(2).unwrap().data = "D";
    /// assert_eq!( tree.to_string(), "a( b c D e )" );
    /// ```
    pub fn nth_child_mut( &mut self, n: usize ) -> Option<&mut Self> {
        let mut pot = self.pot();
        pot.nth_child( self.index(), n ).map( |index| unsafe{ transmute( &mut pot[ index ])})
    }

    /// Provides a forward iterator over child nodes
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let tree: Tree<_> = ( "a","b","c","d" ).into();
    /// let mut iter = tree.root().iter();
    /// assert_eq!( iter.next().unwrap().data, "b" );
    /// assert_eq!( iter.next().unwrap().data, "c" );
    /// assert_eq!( iter.next().unwrap().data, "d" );
    /// assert_eq!( iter.next(), None );
    /// assert_eq!( iter.next(), None );
    /// ```
    pub fn iter<'a, 's:'a>( &'s self ) -> Iter<'a,T> {
        if self.is_leaf() {
            Iter::new( usize::null(), 0, self.pot() )
        } else {
            let pot = self.pot();
            Iter::new( pot.head( self.index() ), self.degree(), pot )
        }
    }

    /// Provides a forward iterator over child nodes with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( 0, 1, 2, 3 ).into();
    /// for child in tree.root_mut().iter_mut() {
    ///     child.data *= 10;
    /// }
    /// assert_eq!( tree.root().to_string(), "0( 10 20 30 )" );
    /// ```
    pub fn iter_mut<'a, 's:'a>( &'s mut self ) -> IterMut<'a,T> {
        if self.is_leaf() {
            IterMut::new( usize::null(), 0, self.pot() )
        } else {
            let index =  self.index();
            let degree =  self.degree();
            let pot = self.pot();
            let first_child = pot.head( index );
            IterMut::new( first_child, degree, pot )
        }
    }

    /// Provides a forward iterator in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::potted::Tree;
    ///
    /// let tree: Tree<_> = (0, (1,2,3), (4,5,6) ).into();
    /// let visits = tree.root().bfs().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: &0, size: Size{ degree: 2, node_cnt: 7 }},
    ///     bfs::Visit{ data: &1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn bfs( &self ) -> BfsTree<Splitted<Iter<T>>> { BfsTree::from( self, Size{ degree: 1, node_cnt: self.size.node_cnt })}

    #[inline] pub(crate) fn inc_sizes( &mut self, degree: usize, node_cnt: usize ) {
        let degree = degree as u32;
        let node_cnt = node_cnt as u32;
        self.size.degree += degree;
        let mut up = self.index();
        let mut pot = self.pot();
        while !up.is_null() {
            pot[ up ].size.node_cnt += node_cnt;
            up = pot.parent( up );
        }
    }

    #[inline] pub(crate) fn dec_sizes( &mut self, degree: usize, node_cnt: usize ) {
        let degree = degree as u32;
        let node_cnt = node_cnt as u32;
        self.size.degree -= degree;
        let mut up = self.index();
        let mut pot = self.pot();
        while !up.is_null() {
            pot[ up ].size.node_cnt -= node_cnt;
            up = pot.parent( up );
        }
    }

    /// Add the tuple tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c" ).into();
    /// tree.root_mut().prepend_tr(( "d", "e", "f" ));
    /// assert_eq!( tree.root().to_string(), "a( d( e f ) b c )" );
    /// ```
    pub fn prepend_tr<Tr>( &mut self, tuple: Tr )
        where Tr: TupleTree<Data=T>
    {
        let tail = self.child();
        self.append_tr( tuple );
        self.set_child( tail );
    }

    /// Add the tuple tree as the last child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c" ).into();
    /// tree.root_mut().append_tr(( "d", "e", "f" ));
    /// assert_eq!( tree.root().to_string(), "a( b c d( e f ) )" );
    /// ```
    pub fn append_tr<Tr>( &mut self, tuple: Tr )
        where Tr: TupleTree<Data=T>
    {
        if self.size.degree == 0 {
            self.adjoined = 1;
        }
        self.inc_sizes( 1, tuple.nodes() );
        tuple.construct_all_nodes( self.index(), self.pot() );
        mem::forget( tuple );
    }

    /// Add the tuple forest as the first-n children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree,fr};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c" ).into();
    /// tree.root_mut().prepend_fr(( fr(), "d", "e", "f" ));
    /// assert_eq!( tree.root().to_string(), "a( d e f b c )" );
    /// ```
    pub fn prepend_fr<Fr>( &mut self, tuple: Fr )
        where Fr: TupleForest<Data=T>
    {
        let tail = self.child();
        self.append_fr( tuple );
        self.set_child( tail );
    }

    /// Add the tuple forest as the last-n children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree,fr};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c" ).into();
    /// tree.root_mut().append_fr(( fr(), "d", "e", "f" ));
    /// assert_eq!( tree.root().to_string(), "a( b c d e f )" );
    /// ```
    pub fn append_fr<Fr>( &mut self, tuple: Fr )
        where Fr: TupleForest<Data=T>
    {
        let trees = tuple.descendants(0);
        if self.size.degree == 0 {
            self.adjoined = trees as u32;
        }
        self.inc_sizes( trees, tuple.nodes() );
        tuple.construct_all_nodes( self.index(), self.pot() );
        mem::forget( tuple );
    }

    /// Insert the tuple tree as the n-th child.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// tree.root_mut().insert_tr( 2, ("f",) );
    /// assert_eq!( tree.root().to_string(), "a( b c f d e )" );
    /// ```
    pub fn insert_tr<Tuple>( &mut self, nth: usize, tuple: Tuple )
        where Tuple: TupleTree<Data=T>
    {
        if nth == 0 {
            self.prepend_tr( tuple );
        } else {
            let degree = self.degree();
            if nth == degree {
                self.append_tr( tuple );
            } else {
                assert!( nth < degree ); // degree > 1
                let prev = self.nth_child( nth-1 ).unwrap().index();
                let tail = self.child();
                self.set_child( prev );
                self.append_tr( tuple );
                self.set_child( tail );
            }
        }
    }

    fn drop_data_recursively( &mut self ) {
        for child in self.iter_mut() {
            child.drop_data_recursively();
        }
        unsafe{ ptr::drop_in_place( &mut self.data )}
    }

    pub(crate) fn drop_all_data_if_needed( &mut self ) {
        if mem::needs_drop::<T>() {
            self.drop_data_recursively();
        }
    }

    pub(crate) fn unlink_back( &mut self ) {
        if !self.is_leaf() {
            let mut pot = self.pot();
            if self.size.degree == 1 {
                self.reset_child();
            } else {
                let back = self.child();

                let new_tail = unsafe{ pot.new_tail( self.index() )};
                let head = pot.head( self.index() );
                pot[ new_tail ].set_next( head );
                pot[ head ].set_prev( new_tail );
                self.set_child( new_tail );

                pot.reset_parent( back );
                pot.reset_sib( back );
                self.dec_sizes( 1, pot.node_cnt( back ));
            }
        }
    }

    /// Drop the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( 1, 2, 3, 4 ).into();
    /// tree.root_mut().drop_front();
    /// assert_eq!( tree.root().to_string(), "1( 3 4 )" );
    /// ```
    pub fn drop_front( &mut self ) {
        let degree = self.degree();
        assert_ne!( degree, 0 );
        if degree == 1 {
            self.drop_back();
        } else { // degree > 1
            let tail = self.child();
            let index = self.index();
            let head = self.pot().head( index );
            self.set_child( head );
            self.drop_back();
            self.set_child( tail );
        }
    }

    /// Drop the last child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( 1, 2, 3, 4 ).into();
    /// tree.root_mut().drop_back();
    /// assert_eq!( tree.root().to_string(), "1( 2 3 )" );
    /// ```
    pub fn drop_back( &mut self ) {
        let degree = self.degree();
        assert_ne!( degree, 0 );
        let tail = self.child();
        let mut pot = self.pot();
        if pot.is_forest( tail ) {
            let forest = tail;
            let forest_tail = pot.tail( forest );
            pot[ forest_tail ].drop_all_data_if_needed();
            pot[ forest ].size.degree -= 1;
            if pot[ forest ].size.degree == 0 {
                pot[ forest ].unlink_back();
            } else {
                self.unlink_back();
            }
        } else {
            if self.adjoined() == degree {
                self.adjoined -= 1;
            }
            pot[ tail ].drop_all_data_if_needed();
            self.unlink_back();
        }
    }

    /// Drop the n-th child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// tree.root_mut().drop_nth( 2 );
    /// assert_eq!( tree.root().to_string(), "a( b c e )" );
    /// ```
    pub fn drop_nth( &mut self, nth: usize ) {
        let degree = self.degree();
        assert_ne!( degree, 0 );
        if nth == 0 {
            self.drop_front();
        } else if nth+1 == degree {
            self.drop_back();
        } else {
            assert!( nth < degree ); // degree > 1
            let tail = self.child();
            let prev = self.pot().nth_child( self.index(), nth ).unwrap();
            self.set_child( prev );
            self.drop_back();
            self.set_child( tail );
        }
    }

    #[inline] fn gather_with_propagation( &mut self, parent: usize, child: usize, data: T, size: Size, do_propagation: bool ) {
        let mut pot = self.pot();
        if do_propagation {
            pot.gather( parent, child, data, Size{ degree: size.degree, node_cnt: 1 });
            pot[ parent ].inc_sizes( 0, 1 );
        } else {
            pot.gather( parent, child, data, size );
        }
    }

    /// Add tree(s) from a bfs iterator as the first child(ren).
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut potted: Tree<_> = ( 0, (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) /tr(6) /tr(7);
    /// potted.root_mut().nth_child_mut(0).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 5( 6 7 ) 2 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) /t(9) /t(10);
    /// potted.root_mut().nth_child_mut(1).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 5( 6 7 ) 2 ) 3( 8( 9 10 ) 4 ) )" );
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut potted: Tree<_> = ( 0, (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) -tr(6) -tr(7);
    /// potted.root_mut().nth_child_mut(0).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 5 6 7 2 ) 3( 4 ) )" );
    ///
    /// //let linked = t(8) /t(9) /t(10);
    /// //potted.root_mut().nth_child_mut(1).unwrap().prepend_bfs( linked.into_bfs() );
    /// //assert_eq!( potted.root().to_string(), "0( 1( 5 6 7 2 ) 3( 8 9 10 4 ) )" );
    /// ```
    pub fn prepend_bfs<Iter>( &mut self, bfs: Bfs<Iter> )
        where Iter : Iterator<Item=Visit<T>>
    {
        let tail = self.child();
        self.append_bfs( bfs );
        self.set_child( tail );
    }

    /// Add tree(s) from a bfs iterator as the last child(ren).
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut potted: Tree<_> = ( 0, (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) /tr(6) /tr(7);
    /// potted.root_mut().nth_child_mut(0).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 2 5( 6 7 ) ) 3( 4 ) )" );
    ///
    /// let linked = t(8) /t(9) /t(10);
    /// potted.root_mut().nth_child_mut(1).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 2 5( 6 7 ) ) 3( 4 8( 9 10 ) ) )" );
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// use trees::linked::singly::tr as t;
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut potted: Tree<_> = ( 0, (1,2), (3,4) ).into();
    ///
    /// let linked = tr(5) -tr(6) -tr(7);
    /// potted.root_mut().nth_child_mut(0).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 2 5 6 7 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) -t(9) -t(10);
    /// potted.root_mut().nth_child_mut(1).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 2 5 6 7 ) 3( 4 8 9 10 ) )" );
    /// ```
    pub fn append_bfs<Iter>( &mut self, bfs: Bfs<Iter> )
        where Iter : Iterator<Item=Visit<T>>
    {
        let ( mut iter, size ) = bfs.iter_and_size();
        let ( degree, node_cnt ) = ( size.degree as usize, size.node_cnt as usize );

        let index  = self.index();

        let do_propagation = node_cnt == 0;
        let mut pot = self.pot();
        let pot_len = pot.len();
        pot.grow( node_cnt );

        let mut parent  = index;
        let mut child   = pot_len;
        let mut remains = degree;

        while let Some( visit ) = iter.next() {
            self.gather_with_propagation( parent, child, visit.data, visit.size, do_propagation );
            remains -= 1;
            while remains == 0 {
                if parent == index {
                    parent = pot_len;
                } else {
                    parent += 1;
                    if parent == child { break; }
                }
                remains = pot.degree( parent );
            }
            child += 1;
        } 
        if do_propagation {
            self.size.degree += degree as u32;
        } else {
            self.inc_sizes( degree, node_cnt );
        }
    }

    /// Provides a forward iterator with mutable references in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::potted::Tree;
    ///
    /// let mut tree: Tree<_> = (0, (1,2,3), (4,5,6) ).into();
    /// let visits = tree.root_mut().bfs_mut().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: &mut 0, size: Size{ degree: 2, node_cnt: 7 }},
    ///     bfs::Visit{ data: &mut 1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &mut 4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &mut 2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn bfs_mut( &mut self ) -> BfsTree<Splitted<IterMut<T>>> {
        let node_cnt = self.size.node_cnt;
        BfsTree::from( self, Size{ degree: 1, node_cnt })
    }
}

impl<T:Debug> Debug for Node<T> {
    fn fmt( &self, fmt: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            write!( fmt, "{:?}", self.data )
        } else {
            write!( fmt, "{:?}( ", self.data )?;
            for child in self.iter() {
                write!( fmt, "{:?} ", child )?;
            }
            write!( fmt, ")" )
        }
    }
}

impl<T:Display> Display for Node<T> {
    fn fmt( &self, fmt: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            write!( fmt, "{}", self.data )
        } else {
            write!( fmt, "{}( ", self.data )?;
            for child in self.iter() {
                write!( fmt, "{} ", child )?;
            }
            write!( fmt, ")" )
        }
    }
}

impl<T:PartialEq> PartialEq for Node<T> {
    fn eq( &self, other: &Self ) -> bool { self.data == other.data && self.iter().eq( other.iter() )}
    fn ne( &self, other: &Self ) -> bool { self.data != other.data || self.iter().ne( other.iter() )}
}

impl<T:Eq> Eq for Node<T> {}

impl<T:PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        match self.data.partial_cmp( &other.data ) {
            None          => None,
            Some( order ) => match order {
                Less    => Some( Less ),
                Greater => Some( Greater ),
                Equal   => self.iter().partial_cmp( other.iter() ),
            },
        }
    }
}

impl<T:Ord> Ord for Node<T> {
    #[inline] fn cmp( &self, other: &Self ) -> Ordering {
        match self.data.cmp( &other.data ) {
            Less    => Less,
            Greater => Greater,
            Equal   => self.iter().cmp( other.iter() ),
        }
    }
}

impl<T:Hash> Hash for Node<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        self.data.hash( state );
        for child in self.iter() {
            child.hash( state );
        }
    }
}

impl<'a, T:'a> Split for &'a Node<T> {
    type Item = &'a T;
    type Iter = Iter<'a,T>;

    fn split( self ) -> ( &'a T, Iter<'a,T>, u32 ) {
        ( &self.data, self.iter(), self.size.node_cnt )
    }
}

impl<'a, T:'a> Split for &'a mut Node<T> {
    type Item = &'a mut T;
    type Iter = IterMut<'a,T>;

    fn split( self ) -> ( &'a mut T, IterMut<'a,T>, u32 ) {
        let node_cnt = self.size.node_cnt;
        (
            unsafe{ transmute( &mut self.data )},
            self.iter_mut(),
            node_cnt
        ) // borrow two mutable references at one time
    }
}

impl<'a, T:'a> IntoIterator for &'a Node<T> {
    type Item = Self;
    type IntoIter = Iter<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        Iter::new( self.index(), 1, self.pot() )
    }
}

impl<'a, T:'a> IntoIterator for &'a mut Node<T> {
    type Item = Self;
    type IntoIter = IterMut<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        IterMut::new( self.index(), 1, self.pot() )
    }
}

pub struct MovedNodes<'a,T,Iter>
    where Iter : Iterator<Item=Visit<&'a T>>
        , T    : 'a
{
    moved : Moved<Iter>,
    pot   : Pot<T>,
    mark  : PhantomData<&'a mut T>,
}

impl<'a,T,Iter> MovedNodes<'a,T,Iter>
    where Iter : Iterator<Item=Visit<&'a T>>
        , T    : 'a
{
    pub(crate) fn new( moved: Moved<Iter>, pot: Pot<T> ) -> Self {
        Self{ moved, pot, mark: PhantomData }
    }
}

impl<'a,T,Iter> Iterator for MovedNodes<'a,T,Iter>
    where Iter : Iterator<Item=Visit<&'a T>>
        , T    : 'a
{
    type Item = Visit<T>;

    fn next( &mut self ) -> Option<Self::Item> { self.moved.next() }
}

impl<'a,T,Iter> Drop for MovedNodes<'a,T,Iter>
    where Iter : Iterator<Item=Visit<&'a T>>
        , T    : 'a
{
    fn drop( &mut self ) {
        for _ in self.by_ref() {}
        unsafe{ Pot::drop( self.pot ); }
    }
}
