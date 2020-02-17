//! Dynamic allocation/deallocation on heap.

use super::{Node,Link,Size};

use crate::rust::*;

pub type Phantom<T> = PhantomData<Box<Node<T>>>;

pub(crate) fn make_node<T>( data: T ) -> *mut Node<T> {
    let mut node = Box::new(
        Node {
            link: Link {
                next   : null_mut(),
                child  : null_mut(),
                prev   : null_mut(),
                parent : null_mut(),
                size   : Size{ degree: 0, node_cnt: 1 },
            },
            data,
        }
    );
    node.reset_sib();
    Box::into_raw( node )
}

pub(crate) fn drop_node<T>( node: *mut Node<T> ) { unsafe{ Box::from_raw( node ); }}
