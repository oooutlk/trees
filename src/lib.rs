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
//! The current version provides two implementions of heap-allocated, child-sibling linked trees.
//! The default implementation is [`linked::fully`](linked/fully/index.html),
//! which stores previous/next sibling and parent/child pointers in one node, with size information tracked.
//! The alternative is [`linked::singly`](linked/singly/index.html),
//! which stores only next sibling and last child pointers in one node, without size information tracked.
//! More kinds of trees will be added in the future.
//!
//! This crate can be used with or without libstd. 
//!
//! ## Quick start
//!
//! 1. `Tree` notation
//!
//!     ```rust,no_run
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
//! 2. `Forest` notation
//!
//!     ```rust,no_run
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
//! 3. `Tree` traversal, using `Node::iter()` recursively
//!
//!     ```rust
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
//!     The `Debug` and `Display` trait has been implemented that is essentially the same as tree_to_tring() mentioned above.
//!
//!     Children are seperated by spaces and grouped in the parentheses that follow their parent closely. 
//!     
//!     ```rust
//!     use trees::{tr,fr};
//!
//!     let tree = tr(0) /( tr(1) /tr(2)/tr(3) ) /( tr(4) /tr(5)/tr(6) );
//!     let str_repr = "0( 1( 2 3 ) 4( 5 6 ) )";
//!     assert_eq!( tree.to_string(), str_repr );
//!     assert_eq!( format!( "{:?}", tree ), str_repr );
//!     
//!     assert_eq!( fr::<i32>().to_string(), "()" );
//!     
//!     let forest = -( tr(1) /tr(2)/tr(3) ) -( tr(4) /tr(5)/tr(6) );
//!     let str_repr = "( 1( 2 3 ) 4( 5 6 ) )";
//!     assert_eq!( forest.to_string(), str_repr );
//!     assert_eq!( format!( "{:?}", fr::<i32>() ), "()" );
//!     ```
//!
//! ## Slow start
//!
//! ### Concepts
//!
//! 1. `Tree` is composed of a root `Node` and an optional `Forest` as its children. A tree can NOT be empty.
//!     ```rust
//!     use trees::{tr,Tree,Forest};
//!
//!     let mut tree: Tree<i32> = tr(0);
//!
//!     let forest: Forest<i32> = -tr(1)-tr(2)-tr(3);
//!     tree.append( forest );
//!     assert_eq!( tree, tr(0) /tr(1) /tr(2) /tr(3) );
//!
//!     { let _forest: &Forest<i32>     = tree.forest();     }
//!     { let _forest: &mut Forest<i32> = tree.forest_mut(); }
//!     { let _forest: Forest<i32>      = tree.abandon();    }
//!
//!     assert_eq!( tree, tr(0) );
//!     ```
//!
//! 2. `Forest` is composed of `Node`s as its children. A forest can be empty.
//!     ```rust,no_run
//!     use trees::{tr,fr,Forest};
//!
//!     let mut forest: Forest<i32> = fr(); // an empty forest
//!     forest.push_back( tr(1) );          // forest has one tree
//!     forest.push_back( tr(2) );          // forest has two trees
//!     ```
//!
//! 3. `Node` is a borrowed tree, and `Tree` is an owned `Node`. All nodes in a tree can be referenced as `&Node`, but only the root node can be observed as `Tree` by the user.
//!     ```rust,no_run
//!     use trees::{tr,Tree,Node};
//!     use std::borrow::Borrow;
//!
//!     let mut tree: Tree<i32>  = tr(0) /tr(1)/tr(2)/tr(3);
//!     {
//!         let root: &Node<i32> = tree.borrow(); // you can also use tree.root()
//!         let first_child : &Node<i32> = tree.iter().next().unwrap();
//!         let second_child: &Node<i32> = tree.iter().nth(1).unwrap();
//!         let third_child : &Node<i32> = tree.iter().last().unwrap();
//!     }
//!     let first_child: Tree<i32> = tree.pop_front().unwrap();
//!     ```
//!
//! ### Iterators
//!
//! The children nodes of a node, or a forest, is conceptually a forward list.
//! 
//! 1. Using `iter()` to iterate over referenced child `Node`s, you can:
//! 
//!     1.1 read the data associated with each node.
//! 
//!     1.2 use `iter()` to iterate over children's children, etc.
//! 
//! 2. Using `iter_mut()` to iterate over referenced child `Node`s, you can:
//! 
//!     2.1 read/write the data associated with each node, or `prepend()`, `append`, `abandon()`, `push_front()`, `pop_front()`, `push_back()` child node(s) in constant time.
//! 
//!     2.2 use `iter_mut()` to iterate over children's children, etc.
//! 
//! 3. Using `onto_iter()` to iterate over `Subnode`s, you can:
//! 
//!     3.1 `insert_before`, `insert_after()`, `depart()` node(s) at any position.
//! 
//!     3.2 do whatever `iter()` or `iter_mut()` can do.
//! 
//! 4. Using `Forest::<T>::into_iter()` to iterate over `Tree`s, you can:
//! 
//!     do whatever you want to.
//! 
//! ### Traversal in depth-first manner
//!
//! Using `TreeWalk`/`ForestWalk` to traverse on `Tree`/`Forest`, you can:
//!
//! 1. read the data associated with each descendant node in depth first manner, preorder or postorder at will.
//!
//! 2. visit `Node`s irregularly, unlike the iterators mentioned above that are usually called intensively.
//! 
//! ### Resource management
//!
//! 1. `Tree`/`Forest` will recursively destruct all the nodes owned by them when reaching the end of their lifetimes.
//!
//! 2. `Clone` for `Tree` and `Forest` makes deep copy which clones all its decendant nodes. To do copy for just one node, simplely `let cloned = trees::tr( node.data.clone() );`.
//!
//! 3. `linked::fully::Node` will track count of children nodes, and count of all descendant nodes and itself, while `linked::singly::node` does not track any size information.
//!
//! ### Traversal in breadth-first manner
//!
//! 1. `Node` provides (mutably) borrowed iterator `fn bfs_iter( &self )`/`fn bfs_iter_mut( &mut self )`.
//!
//! 2. `Tree`/`Forest` provides owned iterator `fn bfs_into_iter( self )`.
//!
//! ### Panics
//!
//! No panics unless `Clone` is involved:
//! * `Node::<T>::to_owned()`
//! * `Tree::<T>::clone()`
//! * `Forest::<T>::clone()`
//! * all of the operator overloading functions the operands of which contain at least one referenced type.
//!
//! Panics if and only if `T::clone()` panics.
//!
//! ### Safety
//!
//! Collections of pointer-based tree implementation require many `unsafe`s to do raw pointer dereferences.
//! Currently this crate contains **150 `unsafe`** lines in its source code.
//!
//! A node's value is composed of its **`data`** field and the **structure** of its descendant nodes.
//! So adding/removing/modifying a child of a node are regarded as modifying its value.
//! And the only possible way to obtain mutable references on a child node is via its parent's mutable reference.
//! Rust borrow checker guarantees memory safety in safe code, and the author of this crate is responsible for memory safety
//! inside `Node`s methods which take `&mut self` and contain `unsafe` blocks.

#![cfg_attr( feature = "no_std", no_std )]
#![cfg_attr( feature = "no_std", feature( alloc ))]

mod rust {
    #[cfg(not(feature="no_std"))] extern crate core;
    #[cfg(not(feature="no_std"))] pub(crate) use std::borrow::{Borrow,BorrowMut};
    #[cfg(not(feature="no_std"))] pub(crate) use std::boxed::Box;
    #[cfg(not(feature="no_std"))] pub(crate) use std::collections::VecDeque;
    #[cfg(not(feature="no_std"))] pub(crate) use std::cmp::Ordering::{self,*};
    #[cfg(not(feature="no_std"))] pub(crate) use std::fmt;
    #[cfg(not(feature="no_std"))] pub(crate) use std::fmt::{Debug,Display,Formatter};
    #[cfg(not(feature="no_std"))] pub(crate) use std::hash::{Hasher,Hash};
    #[cfg(not(feature="no_std"))] pub(crate) use std::iter::{Iterator,FromIterator,IntoIterator,FusedIterator};
    #[cfg(not(feature="no_std"))] pub(crate) use std::marker::PhantomData;
    #[cfg(not(feature="no_std"))] pub(crate) use std::ops::{Deref,DerefMut,Div,Neg,Sub};
    #[cfg(not(feature="no_std"))] pub(crate) use std::ptr::{self,null,null_mut};
    #[cfg(not(feature="no_std"))] pub(crate) use std::vec::Vec;

    #[cfg(feature="no_std")] extern crate alloc;
    #[cfg(feature="no_std")] pub(crate) use self::alloc::borrow::{Borrow,BorrowMut,ToOwned};
    #[cfg(feature="no_std")] pub(crate) use self::alloc::boxed::Box;
    #[cfg(feature="no_std")] pub(crate) use self::alloc::vec::Vec;
    #[cfg(feature="no_std")] pub(crate) use core::collections::VecDeque;
    #[cfg(feature="no_std")] pub(crate) use core::cmp::Ordering::{self,*};
    #[cfg(feature="no_std")] pub(crate) use core::fmt;
    #[cfg(feature="no_std")] pub(crate) use core::fmt::{Debug,Display,Formatter};
    #[cfg(feature="no_std")] pub(crate) use core::hash::{Hasher,Hash};
    #[cfg(feature="no_std")] pub(crate) use core::iter::{Iterator,FromIterator,IntoIterator,FusedIterator};
    #[cfg(feature="no_std")] pub(crate) use core::marker::PhantomData;
    #[cfg(feature="no_std")] pub(crate) use core::ops::{Deref,DerefMut,Div,Neg,Sub};
    #[cfg(feature="no_std")] pub(crate) use core::ptr::{self,null,null_mut};
}

pub mod linked;
pub use linked::{tr,fr,Tree,Forest,Node,Iter,IterMut,Subnode,OntoIter,Visit,TreeWalk,ForestWalk};

//pub mod potted;
pub mod bfs;
