//! `Tree`/`Forest` for trees built top down.
//!
//! Some interesting features of the potted trees are:
//!
//! 1. They can be written in the form of Rust tuple.
//!   Only the tuples composed of no more than a limited fields (now 32) are supported due to the lack of variadic generics in current Rust.
//!   And the library user is required to impl the mark trait `TreeData` for the type other than primitives or strings to be the data type of the tree.
//!
//! 2. The child nodes can be randomly accessed in constant time, as long as the tree/forest is constructed in batch mode, and do few modifications after that.
//!
//! 3. The underlying storage is a `Vec`, which minimize the dynamic memory allocations.

pub mod tree;
pub use self::tree::Tree; 

pub mod forest;
pub use self::forest::Forest; 

pub mod node;
pub use self::node::{NodeRef,NodeMut,MovedNodes,Index};
use self::node::Node;

pub mod pot;
pub use self::pot::Pot;

pub mod iter;
pub use self::iter::{Iter,IterMut};

pub mod notation;
pub use self::notation::{TreeData,TupleTree,TupleForest,fr};

//pub mod cursor;
//pub use self::cursor::Cursor;

pub use super::bfs;
pub use super::Size;

#[cfg(test)]
mod tests {
    use super::*;
    use rust::*;

    #[test] fn test_tree_to_string() {
        fn tree_to_string<'a, T:Display>( node: NodeRef<'a,T> ) -> String {
            if node.is_leaf() {
                node.data().to_string()
            } else {
                format!( "{}( {})", node.data(), 
                    node.iter().fold( String::new(),
                        |s,c| s + &tree_to_string(c) + &" " ))
            }
        }

        let tree: Tree<_> = ( "0", ( "1","2","3" ), ( "4","5","6" ) ).into();
   
        assert_eq!( tree_to_string( tree.root() ), "0( 1( 2 3 ) 4( 5 6 ) )" );
    }

    #[test] fn test_crud() {
        let mut tree: Tree<_> = ( "0", ).into();
        assert_eq!( tree.root().to_string(), "0" );

        tree.root_mut().append_tr(( "1", ));
        assert_eq!( tree.root().to_string(), "0( 1 )" );

        tree.root_mut().append_tr(( "2", "3" ));
        assert_eq!( tree.root().to_string(), "0( 1 2( 3 ) )" );

        tree.root_mut().set_data( "_" );
        assert_eq!( tree.root().to_string(), "_( 1 2( 3 ) )" );

        tree.root_mut().append_tr(( "4", ));
        assert_eq!( tree.root().to_string(), "_( 1 2( 3 ) 4 )" );

        assert_eq!( tree.pot().nth_child( 1, 0 ).unwrap(), 2 );
        assert_eq!( tree.pot().nth_child( 1, 1 ).unwrap(), 3 );
        assert_eq!( tree.pot().nth_child( 1, 2 ).unwrap(), 5 );
        assert_eq!( tree.pot().nth_child( 1, 3 ), None );

        tree.root_mut().nth_child(1).unwrap().drop_back();
        assert_eq!( tree.root().to_string(), "_( 1 2 4 )" );

        tree.root_mut().drop_back();
        assert_eq!( tree.root().to_string(), "_( 1 2 )" );

        tree.root_mut().drop_back();
        assert_eq!( tree.root().to_string(), "_( 1 )" );

        tree.root_mut().drop_back();
        assert_eq!( tree.root().to_string(), "_" );
    }

    #[should_panic]
    #[test]
    fn test_grow() {
        let mut tree: Tree<_> = ( 0, 1, 2 ).into();
        {
            let mut iter = tree.iter_mut();
            let mut first = iter.next().unwrap();
            let mut second = iter.next().unwrap();
            second.append_fr(( fr(), 3, 4, 5, 6, 7 ));
            first.append_fr(( fr(), 8, 9 ));
        }
        let expected: Tree<_> = ( 0, (1,8,9), (2,3,4,5,6,7,) ).into();
        assert_eq!( tree, expected );
    }
}
