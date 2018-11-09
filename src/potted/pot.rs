use super::{Node,Size,NullIndex,TREE,FOREST,NULL,tree_null,forest_null};

use indexed::Pool;

use rust::*;

pub struct Pot<T> {
    pub(crate) nodes: NonNull<Pool<Node<T>>>,
}

impl<T> Copy  for Pot<T> {}
impl<T> Clone for Pot<T> { fn clone( &self ) -> Self { Pot{ nodes: self.nodes }}}

impl<T> Deref for Pot<T> {
    type Target = Pool<Node<T>>;
    fn deref( &self ) -> &Self::Target { unsafe{ self.nodes.as_ref() }}
}

impl<T> DerefMut for Pot<T> {
    fn deref_mut( &mut self ) -> &mut Self::Target { unsafe{ self.nodes.as_mut() }}
}

impl<T> Pot<T> {
    #[inline] pub(crate) fn new_tree() -> Self {
        let mut pool = Pool::new_unmanaged();
        pool.push( tree_null() );
        Pot{ nodes: unsafe{ NonNull::new_unchecked( Box::into_raw( pool ))}}
    }

    #[inline] pub(crate) fn new_forest() -> Self {
        let mut pool = Pool::new_unmanaged();
        pool.push( forest_null() );
        Pot{ nodes: unsafe{ NonNull::new_unchecked( Box::into_raw( pool ))}}
    }

    #[inline] pub(crate) fn grow( &mut self, node_cnt: usize ) {
        let len = self.len();
        let cap = self.capacity();
        if len + node_cnt > cap {
            self.reserve( len + node_cnt - cap );
        }
        unsafe{ self.set_len( len + node_cnt ); }
    }

    // unsafe push_back, no update on parent's `degree` or propagation of `node_cnt`
    #[inline] pub(crate) fn gather( &mut self, parent: usize, child: usize, data: T, size: Size ) {
        let mut node = Node {
            next     : child as u32,
            child    : u32::null(),
            prev     : child as u32,
            parent   : parent as u32,
            size     ,
            adjoined : size.degree,
            index    : child as u32,
            data     ,
        };
        if !parent.is_null() {
            if !self[ parent ].child.is_null() {
                node.set_prev( self.tail( parent ));
                node.set_next( self.head( parent ));
                self.adopt( parent, child, child );
            }
            self[ parent ].set_child( child );
        }
        if self.len() <= child {
            self.grow( 1 );
        }
        unsafe{ self.write( child, node )}
    }

    #[inline] pub(crate) fn nth_child( &self, mut index: usize, mut nth: usize ) -> Option<usize> {
        if nth < self.degree( index ) {
            let mut adjoined = self.adjoined( index );
            index = self.head( index );

            if nth < adjoined { // happy
                return Some( index + nth );
            } else { // sad
                index = self.next( index + adjoined-1 );
                nth -= 1;
                loop {
                    if self.is_forest( index ) {
                        adjoined = self.degree( index );
                        if nth < adjoined {
                            return Some( index + nth );
                        } else {
                            nth -= adjoined;
                            index = self.next( index );
                        }
                    } else {
                        if nth == 0 {
                            return Some( index );
                        } else {
                            nth -= 1;
                            index = self.next( index );
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    #[allow( dead_code )]
    #[inline] pub(crate) fn prev( &self, index: usize ) -> usize { self[ index ].prev as usize }
    #[inline] pub(crate) fn next( &self, index: usize ) -> usize { self[ index ].next as usize }

    // get the actual prev sib node, with "forest node" in mind.
    #[allow( dead_code )]
    #[inline]
    pub(crate) fn prev_sib( &self, index: usize ) -> usize {
        let parent = self.parent( index );
        if parent.is_null() || !self.is_forest( parent ) { // it is inside a normal node
            self.next( index )
        } else { // it is inside a forest node
            if index == self.head( parent ) {
                self.prev( parent )
            } else {
                self.prev( index )
            }
        }
    }

    // get the actual next sib node, with "forest node" in mind.
    #[allow( dead_code )]
    #[inline]
    pub(crate) fn next_sib( &self, index: usize ) -> usize {
        let parent = self.parent( index );
        if parent.is_null() || !self.is_forest( parent ) { // it is inside a normal node
            self.next( index )
        } else { // it is inside a forest node
            if index == self.tail( parent ) {
                self.next( parent )
            } else {
                self.next( index )
            }
        }
    }

    #[inline] pub(crate) fn is_forest_pot( &self ) -> bool { self.is_forest( NULL )}
    #[inline] pub(crate) fn set_tree_pot(   &mut self ) { self[ NULL ].adjoined = TREE; }
    #[inline] pub(crate) fn set_forest_pot( &mut self ) { self[ NULL ].adjoined = FOREST; }

    #[inline] pub(crate) fn adjoined( &self, index: usize )-> usize { self[ index ].adjoined as usize }

    #[inline] pub(crate) fn degree( &self, index: usize ) -> usize { self[ index ].size.degree as usize }

    #[inline] pub(crate) fn node_cnt( &self, index: usize ) -> usize { self[ index ].size.node_cnt as usize }

    #[inline] pub(crate) fn is_leaf( &self, index: usize ) -> bool { self[ index ].child.is_null() }

    #[inline] pub(crate) fn is_forest( &self, index: usize ) -> bool { self[ index ].is_forest() }

    #[inline] pub(crate) fn parent( &self, index: usize ) -> usize { self[ index ].parent as usize }

    #[inline] pub(crate) fn reset_sib( &mut self, index: usize ) { self[ index ].set_prev( index ); self[ index ].set_next( index ); }

    #[inline] pub(crate) fn reset_parent( &mut self, index: usize ) { self[ index ].set_parent( usize::null() ); }

    #[inline] pub(crate) fn tail( &self, index: usize ) -> usize { self[ index ].child() }

    #[inline] pub(crate) fn head( &self, index: usize ) -> usize {
        if self.tail( index ).is_null() {
            usize::null()
        } else {
            self[ self.tail( index ) ].next() // forest is ok to be head as long as all calls of head() is for modifying structure rather than finding a node.
        }
    }

    #[inline] pub(crate) unsafe fn new_tail( &self, index: usize ) -> usize {
        let tail = self.tail( index );
        self[ tail ].prev as usize
    }

    #[inline] pub(crate) fn adopt( &mut self, parent: usize, begin: usize, end: usize ) {
        let parent_head = self.head( parent );
        self[ parent_head ].set_prev( begin );
        let parent_tail = self.tail( parent );
        self[ parent_tail ].set_next( end );
    }

    #[inline] pub(crate) unsafe fn drop( this: Self ) { let _ = Box::from_raw( this.nodes.as_ptr() ); }
}

impl<T:Debug> Debug for Pot<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        let n = if self.is_forest_pot() { 2 } else { 1 };
        for node in self.iter().skip( n ) {
            writeln!( f, "[{}] _{} <{} {}> ({},{}-{}) ^{} {:?}",
                node.index, // [{}]
                node.child, // _{}
                node.prev, node.next, // <{} {}>
                node.size.node_cnt, node.size.degree, node.adjoined, // ({},{}-{})
                node.parent, // ^{}
                node.data    // {:?}
            )?;
        }
        write!( f, "" )
    }
}
