use super::{Node,Size,Index};

use rust::*;

pub struct Pot<T> {
    pub(crate) nodes: Vec<Node<T>>
}

impl<T> Pot<T> {
    #[inline] pub(crate) fn new() -> Self {
        let dummy = Node {
            next     : u32::null(),
            child    : u32::null(),
            prev     : u32::null(),
            parent   : u32::null(),
            size     : Size{ degree: 0, node_cnt: 0 },
            adjoined : 0,
            data     : unsafe{ mem::uninitialized() },
        };
        let mut nodes = Vec::with_capacity( 2 );
        nodes.push( dummy ); // [0] for null
        Pot{ nodes }
    } 

    #[inline] pub(crate) fn grow( &mut self, node_cnt: usize ) {
        let len = self.nodes.len();
        let cap = self.nodes.capacity();
        if len + node_cnt > cap {
            self.nodes.reserve( len + node_cnt - cap );
        }
        unsafe{ self.nodes.set_len( len + node_cnt ); }
    }

    // unsafe version of push_back, no update on parent's `degree` or propagation of `node_cnt`
    #[inline] pub(crate) fn gather( &mut self, parent: usize, child: usize, data: T, size: Size ) {
        let mut node = Node {
            next     : child as u32,
            child    : u32::null(),
            prev     : child as u32,
            parent   : parent as u32,
            size     ,
            adjoined : size.degree,
            data     ,
        };
        if !parent.is_null() {
            if !self.nodes[ parent ].child.is_null() {
                node.set_prev( self.tail( parent ));
                node.set_next( self.head( parent ));
                self.adopt( parent, child, child );
            }
            self.nodes[ parent ].set_child( child );
        }
        if self.nodes.len() <= child {
            self.grow( 1 );
        }
        unsafe{ ptr::write( self.nodes.as_mut_ptr().offset( child as isize ), node )}
    }

    #[inline] pub(crate) fn nth_child( &self, mut index: usize, mut nth: usize ) -> Option<usize> {
        if nth < self.degree( index ) {
            let mut adjoined = self.adjoined( index );
            index = self.head( index );

            if nth < adjoined { // happy
                return Some( index + nth );
            } else {
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

    #[inline] pub(crate) fn next( &self, index: usize ) -> usize { self.nodes[ index ].next as usize }

    #[inline] pub(crate) fn adjoined( &self, index: usize )-> usize { self.nodes[ index ].adjoined as usize }

    #[inline] pub(crate) fn data<'s>( &'s self, index: usize ) -> &'s T { &self.nodes[ index ].data }

    #[inline] pub(crate) fn data_mut<'s>( &'s mut self, index: usize ) -> &mut T { &mut self.nodes[ index ].data }

    #[inline] pub(crate) fn degree( &self, index: usize ) -> usize { self.nodes[ index ].size.degree as usize }

    #[inline] pub(crate) fn node_cnt( &self, index: usize ) -> usize { self.nodes[ index ].size.node_cnt as usize }

    #[inline] pub(crate) fn is_leaf( &self, index: usize ) -> bool { self.nodes[ index ].child.is_null() }

    #[inline] pub(crate) fn is_forest( &self, index: usize ) -> bool { self.nodes[ index ].adjoined == !0 }

    #[inline] pub(crate) fn parent( &self, index: usize ) -> usize { self.nodes[ index ].parent() }

    #[inline] pub(crate) fn child( &self, index: usize ) -> usize { self.nodes[ index ].child() }

    #[inline] pub(crate) fn reset_sib( &mut self, index: usize ) { self.nodes[ index ].set_prev( index ); self.nodes[ index ].set_next( index ); }

    #[inline] pub(crate) fn reset_child(  &mut self, index: usize ) { self.nodes[ index ].set_child(  usize::null() ); }

    #[inline] pub(crate) fn reset_parent( &mut self, index: usize ) { self.nodes[ index ].set_parent( usize::null() ); }

    #[inline] pub(crate) fn tail( &self, index: usize ) -> usize { self.nodes[ index ].child() }

    #[inline] pub(crate) fn head( &self, index: usize ) -> usize {
        if self.tail( index ).is_null() {
            usize::null()
        } else {
            self.nodes[ self.tail( index ) ].next()
        }
    }

    #[inline] pub(crate) unsafe fn new_tail( &self, index: usize ) -> usize {
        let tail = self.tail( index );
        self.nodes[ tail ].prev as usize
    }

    #[inline] pub(crate) fn adopt( &mut self, parent: usize, begin: usize, end: usize ) {
        let parent_head = self.head( parent );
        self.nodes[ parent_head ].set_prev( begin );
        let parent_tail = self.tail( parent );
        self.nodes[ parent_tail ].set_next( end );
    }
}

impl<T:Debug> Debug for Pot<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        let n = if self.nodes[1].next.is_null() { 2 } else { 1 };
        for (index,node) in self.nodes.iter().skip(n).enumerate() {
            writeln!( f, "{} {:?}", index+n, node )?;
        }
        write!( f, "" )
    }
}
