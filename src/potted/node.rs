use super::{Pot,Size,Iter,IterMut,TupleTree,TupleForest};
use super::bfs::{Bfs,BfsTree,Splitted,Split,Moved,Visit};

use rust::*;

pub trait Index {
    fn null() -> Self;
    fn is_null( self ) -> bool;
}

impl Index for u32 {
    fn null() -> Self { 0 }
    fn is_null( self ) -> bool { self == 0 }
}

impl Index for usize {
    fn null() -> Self { 0 }
    fn is_null( self ) -> bool { self == 0 }
}

#[derive(Debug)]
pub(crate) struct Node<T> {
    pub(crate) next     : u32,  // next sibling
    pub(crate) child    : u32,  // last child
    pub(crate) prev     : u32,  // next sibling
    pub(crate) parent   : u32,  // last child
    pub(crate) size     : Size, // count of children and count of all nodes, including itself and all its descendants
    pub(crate) adjoined : u32,  // count of adjioned children.
                                // `adjoined == !0` means "this node has no data, it is a forest of which children are all adjoined"
    pub(crate) data     : T,
}

impl<T> Node<T> {
    pub(crate) fn next(   &self ) -> usize { self.next   as usize }
  //pub(crate) fn prev(   &self ) -> usize { self.prev   as usize }
    pub(crate) fn child(  &self ) -> usize { self.child  as usize }
    pub(crate) fn parent( &self ) -> usize { self.parent as usize }

    pub(crate) fn set_next(   &mut self, index: usize ) { self.next   = index as u32; }
    pub(crate) fn set_prev(   &mut self, index: usize ) { self.prev   = index as u32; }
    pub(crate) fn set_child(  &mut self, index: usize ) { self.child  = index as u32; }
    pub(crate) fn set_parent( &mut self, index: usize ) { self.parent = index as u32; }
}

#[derive(Debug,PartialEq,Eq)]
pub struct NodeRef<'a, T:'a> {
    index : usize,
    pot   : *const Pot<T>,
    mark  : PhantomData<&'a Node<T>>,
}

impl<'a, T:'a> Clone for NodeRef<'a,T> {
    fn clone( &self ) -> Self { NodeRef{ ..*self }}
}

impl<'a, T:'a> Copy for NodeRef<'a,T> {}

impl<'a, T:'a> NodeRef<'a,T> {
    #[inline] pub(crate) fn new( index: usize, pot: *const Pot<T> ) -> Self { Self{ index, pot, mark: PhantomData }}

    #[inline] pub(crate) fn pot<'s>( self ) -> &'s Pot<T> where Self: 's { unsafe{ &*self.pot }}

    #[inline] pub(crate) fn node( &self ) -> &Node<T> { &self.pot().nodes[ self.index ]}

    #[inline] pub fn data( self ) -> &'a T { self.pot().data( self.index )}

    #[inline] pub fn is_leaf( self ) -> bool { self.pot().is_leaf( self.index )}

    /// Returns the n-th child node in between constant time and linear time.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// assert_eq!( tree.root().nth_child(2).unwrap().data(), &"d" );
    /// ```
    pub fn nth_child( self, n: usize ) -> Option<Self> {
        self.pot().nth_child( self.index, n ).map( |index| Self{ index, ..self })
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
    /// assert_eq!( iter.next().unwrap().data(), &"b" );
    /// assert_eq!( iter.next().unwrap().data(), &"c" );
    /// assert_eq!( iter.next().unwrap().data(), &"d" );
    /// assert_eq!( iter.next(), None );
    /// assert_eq!( iter.next(), None );
    /// ```
    pub fn iter( self ) -> Iter<'a,T> where Self: 'a {
        if self.pot().is_leaf( self.index ) {
            Iter::new( usize::null(), 0, self.pot )
        } else {
            Iter::new( self.pot().head( self.index ), self.pot().degree( self.index ), self.pot )
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
    pub fn bfs<'s>( self ) -> BfsTree<Splitted<Iter<'s,T>>> where Self: 's { BfsTree::from( self, Size{ degree: 1, node_cnt: self.node().size.node_cnt })}
}

impl<'a, T:Display> Display for NodeRef<'a,T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            self.data().fmt(f)
        } else {
            self.data().fmt(f)?;
            write!( f, "( " )?;
            for child in self.iter() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
pub struct NodeMut<'a, T:'a> {
    index : usize,
    pot   : *mut Pot<T>,
    mark  : PhantomData<&'a mut Node<T>>,
}

impl<'a, T:'a> NodeMut<'a,T> {
    #[inline] pub(crate) fn new( index: usize, pot: *mut Pot<T> ) -> Self { Self{ index, pot, mark: PhantomData }}

    #[inline] pub(crate) fn pot_mut<'s>( &self ) -> &'s mut Pot<T> where Self: 's { unsafe{ &mut *self.pot }}

    #[inline] pub(crate) fn node( &self ) -> &mut Node<T> { &mut self.pot_mut().nodes[ self.index ]}

    #[inline] pub fn node_mut( &self, index: usize ) -> Self { Self{ index, ..*self }}

    #[inline] pub fn node_ref( &self ) -> NodeRef<'a,T> { NodeRef{ index: self.index, pot: self.pot, mark: PhantomData }}

    #[inline] pub fn set_data( &self, data: T ) { *self.pot_mut().data_mut( self.index ) = data; }

    #[inline] pub fn data_mut( &self ) -> &'a mut T { self.pot_mut().data_mut( self.index )}

    /// Returns the n-th mutable child node in between constant time and linear time.
    /// Note that it is zero-based index.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// let mut tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// tree.root_mut().nth_child(2).unwrap().set_data( "D" );
    /// assert_eq!( tree.root().to_string(), "a( b c D e )" );
    /// ```
    #[inline] pub fn nth_child( self, n: usize ) -> Option<Self> {
        self.pot_mut().nth_child( self.index, n ).map( |index| Self{ index, ..self })
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
    ///     child.set_data( (*child.node_ref().data()) * 10 );
    /// }
    /// assert_eq!( tree.root().to_string(), "0( 10 20 30 )" );
    /// ```
    pub fn iter_mut( &self ) -> IterMut<'a,T> where Self: 'a {
        if self.node_ref().pot().is_leaf( self.index ) {
            IterMut::new( usize::null(), 0, self.pot )
        } else {
            IterMut::new( self.pot_mut().head( self.index ), self.pot_mut().degree( self.index ), self.pot )
        }
    }

    #[inline] pub(crate) fn inc_sizes( &self, degree: usize, node_cnt: usize ) {
        let degree = degree as u32;
        let node_cnt = node_cnt as u32;
        let pot = self.pot_mut();
        let node = self.node();
        node.size.degree += degree;
        let mut up = self.index;
        while !up.is_null() {
            pot.nodes[ up ].size.node_cnt += node_cnt;
            up = pot.parent( up );
        }
    }

    #[inline] pub(crate) fn dec_sizes( &self, degree: usize, node_cnt: usize ) {
        let degree = degree as u32;
        let node_cnt = node_cnt as u32;
        let pot = self.pot_mut();
        self.node().size.degree -= degree;
        let mut up = self.index;
        while !up.is_null() {
            pot.nodes[ up ].size.node_cnt -= node_cnt;
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
    pub fn prepend_tr<Tr>( &self, tuple: Tr )
        where Tr: TupleTree<Data=T>
    {
        let pot = self.pot_mut();
        let tail = pot.tail( self.index );
        self.append_tr( tuple );
        self.node().set_child( tail );
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
    pub fn append_tr<Tr>( &self, tuple: Tr )
        where Tr: TupleTree<Data=T>
    {
        let pot = self.pot_mut();
        if pot.degree( self.index ) == 0 {
            self.node().adjoined = 1;
        }
        self.inc_sizes( 1, tuple.nodes() );
        tuple.construct_all_nodes( self.index, pot );
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
    pub fn prepend_fr<Fr>( &self, tuple: Fr )
        where Fr: TupleForest<Data=T>
    {
        let pot = self.pot_mut();
        let tail = pot.tail( self.index );
        self.append_fr( tuple );
        self.node().set_child( tail );
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
    pub fn append_fr<Fr>( &self, tuple: Fr )
        where Fr: TupleForest<Data=T>
    {
        let pot = self.pot_mut();
        let trees = tuple.descendants(0);
        if pot.degree( self.index ) == 0 {
            self.node().adjoined = trees as u32;
        }
        self.inc_sizes( trees, tuple.nodes() );
        tuple.construct_all_nodes( self.index, pot );
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
    pub fn insert_tr<Tuple>( &self, nth: usize, tuple: Tuple )
        where Tuple: TupleTree<Data=T>
    {
        if nth == 0 {
            self.prepend_tr( tuple );
        } else {
            let pot = self.pot_mut();
            let degree = pot.degree( self.index );
            if nth + 1 == degree {
                self.append_tr( tuple );
            } else {
                assert!( nth < degree ); // degree > 1
                let tail = pot.tail( self.index );
                let prev = pot.nth_child( self.index, nth-1 ).unwrap();
                {
                    self.node().set_child( prev );
                    self.append_tr( tuple );
                    self.node().set_child( tail );
                }
            }
        }
    }

    fn clone( &self ) -> Self { Self{ ..*self }}

    fn drop_data_recursively( &self ) {
        for child in self.clone().iter_mut() {
            child.drop_data_recursively();
        }
        unsafe{ ptr::drop_in_place( self.clone().data_mut() )}
    }

    pub(crate) fn drop_all_data_if_needed( &self ) {
        if mem::needs_drop::<T>() {
            self.drop_data_recursively();
        }
    }

    pub(crate) fn unlink_back( &self ) {
        if !self.node_ref().is_leaf() {
            let pot = self.pot_mut();
            let back = pot.child( self.index );
            if pot.degree( self.index ) == 1 {
                pot.reset_child( self.index );
            } else {
                let new_tail = unsafe{ pot.new_tail( self.index )};
                let head = pot.head( self.index );
                pot.nodes[ new_tail ].set_next( head );
                pot.nodes[ head ].set_prev( new_tail );
                self.node().set_child( new_tail );
            }
            pot.reset_parent( back );
            pot.reset_sib( back );
            self.dec_sizes( 1, pot.node_cnt( back ));
        }
    }

    /// Drop the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// //let mut tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// //tree.root_mut().drop_back();
    /// //assert_eq!( tree.root().to_string(), "a( c d e )" );
    /// ```
    pub fn drop_front( &self ) {
        let pot = &mut *self.pot_mut();
        let degree = pot.degree( self.index );
        assert_ne!( degree, 0 );
        if degree == 1 {
            self.drop_back();
        } else { // degree > 1
            let tail = pot.tail( self.index );
            let head = pot.head( self.index );
            self.node().set_child( head );
            self.drop_back();
            self.node().set_child( tail );
        }
    }

    /// Drop the last child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::potted::{Tree,TreeData,TupleTree};
    ///
    /// //let mut tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// //tree.root_mut().drop_back();
    /// //assert_eq!( tree.root().to_string(), "a( b c d )" );
    /// ```
    pub fn drop_back( &self ) {
        let pot = &mut *self.pot_mut();
        let degree = pot.degree( self.index );
        assert_ne!( degree, 0 );
        let tail = pot.tail( self.index );
        if pot.is_forest( tail ) {
            let forest = tail;
            let forest_tail = pot.tail( forest );
            self.node_mut( forest_tail ).drop_all_data_if_needed();
            pot.nodes[ forest ].size.degree -= 1;
            if pot.nodes[ forest ].size.degree == 0 {
                self.node_mut( forest ).unlink_back();
            } else {
                self.unlink_back();
            }
        } else {
            if pot.adjoined( self.index ) == degree {
                self.node().adjoined -= 1;
            }
            self.node_mut( tail ).drop_all_data_if_needed();
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
    /// //let mut tree: Tree<_> = ( "a", "b", "c", "d", "e" ).into();
    /// //tree.root_mut().drop_nth( 2 );
    /// //assert_eq!( tree.root().to_string(), "a( b c e )" );
    /// ```
    pub fn drop_nth( &self, nth: usize ) {
        let pot = &mut *self.pot_mut();
        let degree = pot.degree( self.index );
        assert_ne!( degree, 0 );
        if nth == 0 {
            self.drop_front();
        } else if nth+1 == degree {
            self.drop_back();
        } else {
            assert!( nth < degree ); // degree > 1
            let tail = pot.tail( self.index );
            let prev = pot.nth_child( self.index, nth-1 ).unwrap();
            {
                self.node().set_child( prev );
                self.drop_back();
                self.node().set_child( tail );
            }
        }
    }

    #[inline] fn gather_with_propagation( &self, parent: usize, child: usize, data: T, size: Size, do_propagation: bool ) {
        if do_propagation {
            self.pot_mut().gather( parent, child, data, Size{ degree: size.degree, node_cnt: 1 });
            self.node_mut( parent ).inc_sizes( 0, 1 );
        } else {
            self.pot_mut().gather( parent, child, data, size );
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
    /// potted.root_mut().nth_child(0).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 5( 6 7 ) 2 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) /t(9) /t(10);
    /// potted.root_mut().nth_child(1).unwrap().prepend_bfs( linked.into_bfs().wrap() );
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
    /// potted.root_mut().nth_child(0).unwrap().prepend_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 5 6 7 2 ) 3( 4 ) )" );
    ///
    /// //let linked = t(8) /t(9) /t(10);
    /// //potted.root_mut().nth_child(1).unwrap().prepend_bfs( linked.into_bfs() );
    /// //assert_eq!( potted.root().to_string(), "0( 1( 5 6 7 2 ) 3( 8 9 10 4 ) )" );
    /// ```
    pub fn prepend_bfs<Iter>( &self, bfs: Bfs<Iter> )
        where Iter : Iterator<Item=Visit<T>>
    {
        let pot = self.pot_mut();
        let tail = pot.tail( self.index );
        self.append_bfs( bfs );
        self.node().set_child( tail );
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
    /// potted.root_mut().nth_child(0).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 2 5( 6 7 ) ) 3( 4 ) )" );
    ///
    /// let linked = t(8) /t(9) /t(10);
    /// potted.root_mut().nth_child(1).unwrap().append_bfs( linked.into_bfs().wrap() );
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
    /// potted.root_mut().nth_child(0).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 2 5 6 7 ) 3( 4 ) )" );
    ///
    /// let linked = t(8) -t(9) -t(10);
    /// potted.root_mut().nth_child(1).unwrap().append_bfs( linked.into_bfs().wrap() );
    /// assert_eq!( potted.root().to_string(), "0( 1( 2 5 6 7 ) 3( 4 8 9 10 ) )" );
    /// ```
    pub fn append_bfs<Iter>( &self, bfs: Bfs<Iter> )
        where Iter : Iterator<Item=Visit<T>>
    {
        let ( mut iter, size ) = bfs.iter_and_size();
        let ( degree, node_cnt ) = ( size.degree as usize, size.node_cnt as usize );

        let do_propagation = node_cnt == 0;
        let pot_len = self.pot_mut().nodes.len();
        self.pot_mut().grow( node_cnt );

        let mut parent  = self.index;
        let mut child   = pot_len;
        let mut remains = degree;

        while let Some( visit ) = iter.next() {
            self.gather_with_propagation( parent, child, visit.data, visit.size, do_propagation );
            remains -= 1;
            while remains == 0 {
                if parent == self.index {
                    parent = pot_len;
                } else {
                    parent += 1;
                    if parent == child { break; }
                }
                remains = self.node_ref().pot().degree( parent );
            }
            child += 1;
        } 
        if do_propagation {
            self.node().size.degree += degree as u32;
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
    pub fn bfs_mut<'s>( self ) -> BfsTree<Splitted<IterMut<'s,T>>> where Self: 's {
        let node_cnt = self.node().size.node_cnt;
        BfsTree::from( self, Size{ degree: 1, node_cnt })
    }
}

impl<'a, T:'a> Split for NodeRef<'a,T> {
    type Item = &'a T;
    type Iter = Iter<'a,T>;

    fn split( self ) -> ( &'a T, Iter<'a,T>, u32 ) {
        ( self.data(), self.iter(), self.node().size.node_cnt )
    }
}

impl<'a, T:'a> Split for NodeMut<'a,T> {
    type Item = &'a mut T;
    type Iter = IterMut<'a,T>;

    fn split( self ) -> ( &'a mut T, IterMut<'a,T>, u32 ) {
        let node_cnt = self.node().size.node_cnt;
        unsafe{ ( &mut *( self.data_mut() as *mut T ), self.iter_mut(), node_cnt )} // borrow two mutable references at one time
    }
}

impl<'a, T:'a> IntoIterator for NodeRef<'a,T> {
    type Item = Self;
    type IntoIter = Iter<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        Iter::new( self.index, 1, self.pot )
    }
}

impl<'a, T:'a> IntoIterator for NodeMut<'a,T> {
    type Item = Self;
    type IntoIter = IterMut<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        IterMut::new( self.index, 1, self.pot )
    }
}

pub struct MovedNodes<'a,T,Iter>
    where Iter : Iterator<Item=Visit<&'a T>>
        , T    : 'a
{
    moved : Moved<Iter>,
    nodes : *mut Vec<Node<T>>,
    mark  : PhantomData<&'a mut T>,
}

impl<'a,T,Iter> MovedNodes<'a,T,Iter>
    where Iter : Iterator<Item=Visit<&'a T>>
        , T    : 'a
{
    pub(crate) fn new( moved: Moved<Iter>, nodes: &mut Vec<Node<T>> ) -> Self {
        let nodes = nodes as *mut _;
        Self{ moved, nodes, mark: PhantomData }
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
        unsafe {
            let cap = (*self.nodes).capacity();
            let _ = Vec::from_raw_parts( self.nodes, 0, cap );
        }
    }
}
