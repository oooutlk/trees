// Copyright 2018 oooutlk@outlook.com. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # trees
//!
//! General purpose tree library.
//!
//! The current version provides two implementions of heap-allocated, child-sibling linked trees and one implementation of vec-backed tree.
//!
//! - The default implementation is [linked::fully](linked/fully/index.html),
//! which stores previous/next sibling and parent/child pointers in one node, with size information tracked.
//!
//! - The alternative linked tree is [linked::singly](linked/singly/index.html),
//! which stores only next sibling and last child pointers in one node, without size information tracked.
//! The space cost is minimal, but with a few penalties on time cost or lack of function, e.g. linear time size_hint of iterators, and missing pop_back().
//!
//! - The other alternative using vec as its underlying storage is [potted](potted/index.html). 
//! The memory allocations are minimal, and **trees can be written in Rust tuples**.
//! Random access over child nodes is supported for tree/forest constructed in batch mode.
//!
//! More kinds of trees will be added in the future.
//!
//! This crate can be used with or without libstd. 
//!
//! ## Quick start
//!
//! 1. Tree notation
//!
//!     ```no_run
//!     use trees::tr;      // tr stands for tree
//!     tr(0);              // A single tree node with data 0. tr(0) has no children
//!     tr(0) /tr(1);       // tr(0) has one child tr(1)
//!     tr(0) /tr(1)/tr(2); // tr(0) has children tr(1) and tr(2)
//!     
//!     // tr(0) has children tr(1) and tr(4), while tr(1) has children tr(2) and tr(3), and tr(4) has children tr(5) and tr(6).
//!     // The spaces and carriage returns are for pretty format and do not make sense.
//!     tr(0)
//!         /( tr(1) /tr(2)/tr(3) )
//!         /( tr(4) /tr(5)/tr(6) );
//!     ```
//!
//!     The potted version:
//!     ```no_run
//!     // the trees written in potted tree, same as described above
//!     use trees::potted::{Tree,TreeData,TupleTree};
//!     Tree::from(( 0, ));
//!     Tree::from(( 0, 1 ));
//!     Tree::from(( 0, 1, 2 ));
//!     Tree::from(( 0, (1, 2, 3), (4, 5, 6)));
//!     ```
//!
//! 2. Forest notation
//!
//!     ```no_run
//!     use trees::{tr,fr}; // fr stands for forest
//!
//!     fr::<i32>();        // An empty forest
//!     fr() - tr(1);       // forest has one child tr(1)
//!     - tr(1);            // forest has one child tr(1). The fr() can be omitted. The Neg operator for Tree converts the tree to a forest.
//!     - tr(1) - tr(2);    // forest has child tr(1) and tr(2)
//!     tr(1) - tr(2);      // forest has child tr(1) and tr(2). The leading neg can be omitted.
//!     
//!     // forest has children tr(1) and tr(4), while tr(1) has children tr(2) and tr(3), and tr(4) has children tr(5) and tr(6).
//!     -( tr(1) /tr(2)/tr(3) )
//!     -( tr(4) /tr(5)/tr(6) );
//!
//!     // A tree tr(0) whose children equal to the forest descripted above.
//!     tr(0) /(
//!         -( tr(1) /( -tr(2)-tr(3) ) )
//!         -( tr(4) /( -tr(5)-tr(6) ) )
//!     );
//!     ```
//!
//!     The potted version:
//!     ```no_run
//!     // the forests written in potted tree, same as described above
//!     use trees::potted::{Forest,TreeData,TupleForest,fr};
//!
//!     Forest::<i32>::new(); Forest::<i32>::from(( fr(), ));
//!     Forest::from(( fr(), 1 ));
//!     Forest::from(( fr(), 1, 2 ));
//!     Forest::from(( fr(), (1,2,3), (4,5,6) ));
//!     ```
//!
//! 3. Tree traversal, using Node::iter() recursively
//!
//!     ```
//!     use trees::{tr,Node};
//!     use std::fmt::Display;
//!
//!     let tree = tr(0)
//!         /( tr(1) /tr(2)/tr(3) )
//!         /( tr(4) /tr(5)/tr(6) );
//!
//!     fn tree_to_string<T:Display>( node: &Node<T> ) -> String {
//!         if node.is_leaf() {
//!             node.data.to_string()
//!         } else {
//!             format!( "{}( {})", node.data, 
//!                 node.iter().fold( String::new(),
//!                     |s,c| s + &tree_to_string(c) + &" " ))
//!         }
//!     }
//!
//!     assert_eq!( tree_to_string( &tree ), "0( 1( 2 3 ) 4( 5 6 ) )" );
//!     ```
//!
//! 4. String representation 
//! 
//! The Display trait has been implemented that is essentially the same as tree_to_tring() mentioned above.
//!
//! Children are seperated by spaces and grouped in the parentheses that follow their parent closely. 
//!     
//!```
//!     use trees::{tr,fr};
//!
//!     let tree = tr(0) /( tr(1) /tr(2)/tr(3) ) /( tr(4) /tr(5)/tr(6) );
//!     let str_repr = "0( 1( 2 3 ) 4( 5 6 ) )";
//!     assert_eq!( tree.to_string(), str_repr );
//!     
//!     assert_eq!( fr::<i32>().to_string(), "()" );
//!     
//!     let forest = -( tr(1) /tr(2)/tr(3) ) -( tr(4) /tr(5)/tr(6) );
//!     let str_repr = "( 1( 2 3 ) 4( 5 6 ) )";
//!     assert_eq!( forest.to_string(), str_repr );
//!```
//!
//! ## Slow start
//!
//! ### Concepts
//!
//! 1. Tree is composed of a root Node and an optional Forest as its children. A tree can NOT be empty.
//!     ```
//!     use trees::{tr,Tree,Forest};
//!     use std::pin::Pin;
//!
//!     let mut tree: Tree<i32> = tr(0);
//!
//!     let forest: Forest<i32> = -tr(1)-tr(2)-tr(3);
//!     tree.root_mut().append( forest );
//!     assert_eq!( tree, tr(0) /tr(1) /tr(2) /tr(3) );
//!
//!     { let _forest: &Forest<i32>          = tree.forest();                }
//!     { let _forest: Pin<&mut Forest<i32>> = tree.root_mut().forest_mut(); }
//!     { let _forest: Forest<i32>           = tree.abandon();               }
//!
//!     assert_eq!( tree, tr(0) );
//!     ```
//!
//!     The potted version:
//!     ```
//!     // potted::Forest cannot be borrowed from potted::Tree, and abandon is different.
//!     use trees::potted::{Tree,Forest,TreeData,TupleTree,TupleForest,fr};
//!
//!     let mut forest = Forest::from(( fr(), 1, 2, 3 ));
//!     let mut tree = forest.adopt( 0 );
//!     assert_eq!( tree.to_string(), "0( 1 2 3 )" );
//!     let ( root_data, forest ) = tree.abandon();
//!     assert_eq!( root_data, 0 );
//!     assert_eq!( forest.to_string(), "( 1 2 3 )" );
//!     ```
//!
//! 2. Forest is composed of Nodes as its children. A forest can be empty.
//!     ```no_run
//!     use trees::{tr,fr,Forest};
//!
//!     let mut forest: Forest<i32> = fr(); // an empty forest
//!     forest.push_back( tr(1) );          // forest has one tree, tr(1)
//!     forest.push_back( tr(2) );          // forest has two trees, tr(1) and tr(2)
//!     ```
//!
//!     The potted version:
//!     ```no_run
//!     use trees::potted::{Tree,Forest,TreeData,TupleForest};
//!     let mut forest = Forest::<i32>::new(); // an empty forest
//!     forest.append_tr(( 1, 2, 3 ));         // forest has three nodes
//!     ```
//!
//! 3. Node is a borrowed tree, and Tree is an owned Node. All nodes in a tree can be referenced as &Node, but only the root node can be observed as Tree by the user.
//!     ```no_run
//!     use trees::{tr,Tree,Node};
//!     use std::borrow::Borrow;
//!
//!     let mut tree: Tree<i32> = tr(0) /tr(1)/tr(2)/tr(3);
//!     {
//!         let root: &Node<i32> = tree.borrow(); // you can also use tree.root()
//!         let first_child : &Node<i32> = tree.iter().next().unwrap();
//!         let second_child: &Node<i32> = tree.iter().nth(1).unwrap();
//!         let third_child : &Node<i32> = tree.iter().last().unwrap();
//!     }
//!     let first_child: Tree<i32> = tree.root_mut().pop_front().unwrap();
//!     ```
//!
//!     The potted version:
//!     ```no_run
//!     use trees::potted::{Tree,Node,TreeData,TupleTree};
//!     let mut tree = Tree::from(( 0, 1, 2, 3 ));
//!     {
//!         let root: &Node<i32> = tree.root();
//!         let first_child : &Node<i32> = tree.root().iter().next().unwrap();
//!         let second_child: &Node<i32> = tree.root().nth_child(1).unwrap(); // nth_child() is in constant time.
//!         let third_child : &Node<i32> = tree.root().iter().last().unwrap();
//!     }
//!     ```
//!
//! ### Iterators
//!
//! The children nodes of a node, or a forest, is conceptually a forward list.
//! 
//! 1. Using iter() to iterate over referenced child Nodes, you can:
//! 
//! - read the data associated with each node.
//! 
//! - use iter() to iterate over children's children, etc.
//! 
//! 2. Using iter_mut() to iterate over referenced child Nodes, you can:
//! 
//! - read/write the data associated with each node, or prepend(), append, abandon(), push_front(), pop_front(), push_back(), pop_back() child node(s) in constant time.
//!
//! Note that linked::singly does not have pop_back(), and potted tree/forest's methods are different in names and/or functionalities.
//! 
//! - use iter_mut() to iterate over children's children, etc.
//! 
//! 3. Using onto_iter() to iterate over Subnodes, you can:
//! 
//! - insert_before, insert_after(), depart() node(s) at any position.
//! 
//! - do whatever iter() or iter_mut() can do.
//! 
//! Note that it is not implemented for potted version.
//! 
//! 4. Using Forest::<T>::into_iter() to iterate over Trees, you can:
//! 
//! - Do whatever you want to.
//! 
//! Note that it is not implemented for potted version.
//! 
//! ### Traversal in depth-first manner
//!
//! Using TreeWalk/ForestWalk to traverse on Tree/Forest, you can:
//!
//! 1. read the data associated with each descendant node in depth first manner, preorder or postorder at will.
//!
//! 2. visit Nodes irregularly, unlike the iterators mentioned above that are usually called intensively.
//! 
//! Note that it is not implemented yet for potted version.
//!
//! ### Resource management
//!
//! 1. Tree/Forest will recursively destruct all the nodes owned by them when reaching the end of their lifetimes.
//!
//! 2. Clone for Tree and Forest makes deep copy which clones all its decendant nodes. To do copy for just one node, simplely let cloned = trees::tr( node.data.clone() );.
//!
//! 3. linked::fully::Node will track count of children nodes, and count of all descendant nodes and itself, while linked::singly::node does not track any size information.
//!
//! ### Traversal in breadth-first manner
//!
//! 1. Node provides (mutably) borrowed iterator fn bfs_iter( &self )/fn bfs_iter_mut( &mut self ).
//!
//! 2. Tree/Forest provides owned iterator fn bfs_into_iter( self ).
//!
//! 3. All version of Tree/Forest/Node support Into BFS streams, while potted version supports From BFS streams also.
//!
//! ### Panics
//!
//! One cause of panics is tree data's Clone:
//! * Node::<T>::to_owned()
//! * Tree::<T>::clone()
//! * Forest::<T>::clone()
//! * all of the operator overloading functions the operands of which contain at least one referenced type.
//!
//! A few assertions in potted version can also cause panics.
//!
//! ### Safety
//!
//! Collections of pointer-based tree implementation require many unsafes to do raw pointer dereferences.
//! Currently this crate contains **200+ unsafe** blocks in its source code.
//! This crate relies on lifetime bounds and borrow check to keep memory-safety, in compile time.
//! The following are some simple demonstrations.
//!
//! ```compile_fail
//! use trees::tr;
//!
//! let root; // node reference can not live longer than tree
//! {
//!     let tree = tr(0);
//!     root = tree.root();
//! }
//! root.push_back( tr(1) );
//! ```
//!
//! ```compile_fail
//! use trees::tr;
//!
//! let root; // mutable node reference can not longer than tree
//! {
//!     let mut tree = tr(0);
//!     root = tree.root_mut();
//! }
//! root.pop_front();
//! ```
//!
//! ```compile_fail
//! use trees::tr;
//!
//! let mut tree = tr(0) /tr(1);
//! let child = tree.iter().next();
//! tree.abandon(); // can not drop sub trees being borrowed
//! let _ = child.first();
//! ```
//!
//! ```compile_fail
//! use trees::{Node,tr};
//!
//! let mut tree = tr(0) /tr(1) /tr(2);
//! let child1 = tree.iter_mut().next().unwrap();
//! let child2 = tree.iter_mut().next().unwrap(); // can not have two mutable references on the same node
//! child2.push_back( tr(3) );
//! child1.push_back( tr(4) );
//! ```

#![cfg_attr( feature = "no_std", no_std )]
#![cfg_attr( feature = "no_std", feature( alloc ))]

extern crate indexed;

mod rust {
    #[cfg(not(feature="no_std"))] pub(crate) use std::borrow::{Borrow,ToOwned};
    #[cfg(not(feature="no_std"))] pub(crate) use std::boxed::Box;
    #[cfg(not(feature="no_std"))] pub(crate) use std::collections::VecDeque;
    #[cfg(not(feature="no_std"))] pub(crate) use std::cmp::Ordering::{self,*};
    #[cfg(not(feature="no_std"))] pub(crate) use std::fmt::{self,Debug,Display,Formatter};
    #[cfg(not(feature="no_std"))] pub(crate) use std::hash::{Hasher,Hash};
    #[cfg(not(feature="no_std"))] pub(crate) use std::iter::{Iterator,FromIterator,IntoIterator,FusedIterator};
    #[cfg(not(feature="no_std"))] pub(crate) use std::marker::{PhantomData,Unpin};
    #[cfg(not(feature="no_std"))] pub(crate) use std::mem::{self,forget,transmute};
    #[cfg(not(feature="no_std"))] pub(crate) use std::ops::{Add,AddAssign,Deref,Div,Neg,Sub,SubAssign};
    #[cfg(not(feature="no_std"))] pub(crate) use std::pin::Pin;
    #[cfg(not(feature="no_std"))] pub(crate) use std::ptr::{self,NonNull,null,null_mut};
    #[cfg(not(feature="no_std"))] pub(crate) use std::vec::Vec;

    #[cfg(feature="no_std")] extern crate core;
    #[cfg(feature="no_std")] extern crate alloc;
    #[cfg(feature="no_std")] pub(crate) use self::alloc::borrow::{Borrow,ToOwned};
    #[cfg(feature="no_std")] pub(crate) use self::alloc::boxed::Box;
    #[cfg(feature="no_std")] pub(crate) use self::alloc::string::String;
    #[cfg(feature="no_std")]
                #[cfg(test)] pub(crate) use self::alloc::string::ToString;
    #[cfg(feature="no_std")] pub(crate) use self::alloc::collections::VecDeque;
    #[cfg(feature="no_std")]
                #[cfg(test)] pub(crate) use self::alloc::format;
    #[cfg(feature="no_std")] pub(crate) use self::alloc::vec::Vec;
    #[cfg(feature="no_std")] pub(crate) use core::cmp::Ordering::{self,*};
    #[cfg(feature="no_std")] pub(crate) use core::fmt::{self,Debug,Display,Formatter};
    #[cfg(feature="no_std")] pub(crate) use core::hash::{Hasher,Hash};
    #[cfg(feature="no_std")] pub(crate) use core::iter::{Iterator,FromIterator,IntoIterator,FusedIterator};
    #[cfg(feature="no_std")] pub(crate) use core::marker::{PhantomData,Unpin};
    #[cfg(feature="no_std")] pub(crate) use core::mem::{self,forget,transmute};
    #[cfg(feature="no_std")] pub(crate) use core::ops::{Add,AddAssign,Deref,Div,Neg,Sub,SubAssign};
    #[cfg(feature="no_std")] pub(crate) use core::pin::Pin;
    #[cfg(feature="no_std")] pub(crate) use core::ptr::{self,NonNull,null,null_mut};
}

pub mod linked;
pub use crate::linked::{tr,fr,Tree,Forest,Node,Iter,IterMut,Subnode,OntoIter,Visit,TreeWalk,ForestWalk};

pub mod potted;

pub mod bfs;

pub mod size;
pub use size::Size;
