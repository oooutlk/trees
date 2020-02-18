//! Dynamic allocation/deallocation on heap.

use super::{Node,Link};

use crate::rust::*;

pub type Phantom<T> = PhantomData<Box<Node<T>>>;

pub(crate) fn make_node<T>( data: T ) -> *mut Node<T> {
    let mut node = Box::new(
        Node {
            link: Link {
                next  : null_mut(),
                child : null_mut(),
            },
            data,
        }
    );
    node.link_mut().reset_sib();
    Box::into_raw( node )
}

pub(crate) fn drop_node<T>( node: *mut Node<T> ) { unsafe{ Box::from_raw( node ); }}
