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
pub use self::node::{Node,MovedNodes,NullIndex};

pub mod pot;
pub use self::pot::Pot;

pub mod iter;
pub use self::iter::{Iter,IterMut};

pub mod notation;
pub use self::notation::{TreeData,TupleTree,TupleForest,fr};

pub use super::bfs;
pub use super::Size;

use rust::*;

const NULL : usize = 0;  // for convenience in constructing tree/forest from BFS stream.
const ROOT : usize = 1;  // root for tree, fake root for forest.
const TREE   : u32 = 0;  // flag in [NULL].adjoined to indicate a potted tree.
const FOREST : u32 = !0; // flag in [NULL].adjoined to indicate a potted forest.

fn tree_null<T>() -> Node<T> {
    Node {
        next     : NULL as u32,
        child    : u32::null(),
        prev     : NULL as u32,
        parent   : u32::null(),
        size     : Size{ degree: 0, node_cnt: 0 },
        adjoined : TREE,
        index    : NULL as u32,
        data     : unsafe{ mem::uninitialized() },
    }
}

fn forest_null<T>() -> Node<T> {
    Node {
        next     : NULL as u32,
        child    : u32::null(),
        prev     : NULL as u32,
        parent   : u32::null(),
        size     : Size{ degree: 0, node_cnt: 0 },
        adjoined : FOREST,
        index    : NULL as u32,
        data     : unsafe{ mem::uninitialized() },
    }
}

fn fake_root <T>() -> Node<T> {
    Node {
        next     : ROOT as u32,
        child    : u32::null(),
        prev     : ROOT as u32,
        parent   : u32::null(),
        size     : Size{ degree: 0, node_cnt: 0 },
        adjoined : 0,
        index    : ROOT as u32,
        data     : unsafe{ mem::uninitialized() },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_tree_to_string() {
        fn tree_to_string<'a, T:Display>( node: &'a Node<T> ) -> String {
            if node.is_leaf() {
                node.data.to_string()
            } else {
                format!( "{}( {})", node.data, 
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

        tree.root_mut().data = "_";
        assert_eq!( tree.root().to_string(), "_( 1 2( 3 ) )" );

        tree.root_mut().append_tr(( "4", ));
        assert_eq!( tree.root().to_string(), "_( 1 2( 3 ) 4 )" );

        assert_eq!( tree.pot().nth_child( 1, 0 ).unwrap(), 2 );
        assert_eq!( tree.pot().nth_child( 1, 1 ).unwrap(), 3 );
        assert_eq!( tree.pot().nth_child( 1, 2 ).unwrap(), 5 );
        assert_eq!( tree.pot().nth_child( 1, 3 ), None );

        tree.root_mut().nth_child_mut(1).unwrap().drop_back();
        assert_eq!( tree.root().to_string(), "_( 1 2 4 )" );

        tree.root_mut().drop_back();
        assert_eq!( tree.root().to_string(), "_( 1 2 )" );

        tree.root_mut().drop_back();
        assert_eq!( tree.root().to_string(), "_( 1 )" );

        tree.root_mut().drop_back();
        assert_eq!( tree.root().to_string(), "_" );
    }

    #[test]
    fn test_grow() {
        let mut tree: Tree<_> = ( 0, 1, 2 ).into();
        {
            let mut iter = tree.iter_mut();
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();
            second.append_fr(( fr(), 3, 4, 5, 6, 7 ));
            first.append_fr(( fr(), 8, 9 ));
        }
        let expected: Tree<_> = ( 0, (1,8,9), (2,3,4,5,6,7,) ).into();
        assert_eq!( tree, expected );
    }
}
