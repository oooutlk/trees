// Copyright 2018 oooutlk@outlook.com. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! General purpose tree library.
//! See the [trees book](https://oooutlk.github.io/trees/) for more.
//!
//! # Examples
//!
//! The code below construct the following tree in different ways:
//!
//! ```text
//! .............
//! .     0     .
//! .   /   \   .
//! .  1     4  .
//! . / \   / \ .
//! .2   3 5   6.
//! .............
//! ```
//!
//! ## Example of `tr` notations for building trees
//!
//! ```rust
//! use trees::tr;
//!
//! let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
//! ```
//!
//! ## Example of tuple notations for building trees
//!
//! ```rust
//! let tree = trees::Tree::<i32>::from_tuple(( 0, (1,2,3), (4,5,6) ));
//! ```
//!
//! ## Example of building trees step by step
//!
//! ```rust
//! use trees::Tree;
//!
//! let mut tree = Tree::new(0);
//!
//! let mut root = tree.root_mut();
//! root.push_back( Tree::new(1) );
//! root.push_back( Tree::new(4) );
//!
//! let mut children = root.iter_mut();
//!
//! let mut node_1 = children.next().unwrap();
//! node_1.push_back( Tree::new(2) );
//! node_1.push_back( Tree::new(3) );
//!
//! let mut node_4 = children.next().unwrap();
//! node_4.push_back( Tree::new(5) );
//! node_4.push_back( Tree::new(6) );
//! ```
//!
//! # Overview of features
//!
//! 1. Step-by-step [creating, reading, updating, deleting](./crud.md) and iterating
//! nodes with assocated data items.
//!
//! 2. Compact notations to express trees: `-`,`/` encoded or tuple encoded trees.
//!
//! 3. Depth first search cursor.
//!
//! 4. Breadth first search iterators.
//!
//! 5. Trees can be built by stages, with nodes stored scatteredly among memory.
//!
//! 6. Trees can be built once through, with nodes stored contiguously.
//!
//! 7. Support exclusive ownership with static borrow check.
//!
//! 8. Support shared ownership with dynamic borrow check.

#![cfg_attr( feature = "no_std", no_std )]

#[doc( hidden )]
pub mod rust {
    #[cfg(not(feature="no_std"))] pub use std::borrow::{Borrow, ToOwned};
    #[cfg(not(feature="no_std"))] pub use std::boxed::Box;
    #[cfg(not(feature="no_std"))] pub use std::cell::{Cell, Ref, RefMut, RefCell};
    #[cfg(not(feature="no_std"))] pub use std::collections::VecDeque;
    #[cfg(not(feature="no_std"))] pub use std::cmp::Ordering::{self, *};
    #[cfg(not(feature="no_std"))] pub use std::fmt::{self, Debug, Display, Formatter};
    #[cfg(not(feature="no_std"))] pub use std::hash::{Hasher, Hash};
    #[cfg(not(feature="no_std"))] pub use std::iter::{Iterator, FromIterator, IntoIterator, FusedIterator};
    #[cfg(not(feature="no_std"))] pub use std::marker::{PhantomData, Unpin};
    #[cfg(not(feature="no_std"))] pub use std::mem::{self, forget, transmute, MaybeUninit};
    #[cfg(not(feature="no_std"))] pub use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Neg, Sub, SubAssign};
    #[cfg(not(feature="no_std"))] pub use std::pin::Pin;
    #[cfg(not(feature="no_std"))] pub use std::ptr::{self, NonNull, null, null_mut};
    #[cfg(not(feature="no_std"))] pub use std::rc::{Rc, Weak};
    #[cfg(not(feature="no_std"))] pub use std::vec::Vec;

    #[cfg(feature="no_std")] extern crate core;
    #[cfg(feature="no_std")] extern crate alloc;
    #[cfg(feature="no_std")] pub use self::alloc::borrow::{Borrow, ToOwned};
    #[cfg(feature="no_std")] pub use self::alloc::boxed::Box;
    #[cfg(feature="no_std")] pub use self::alloc::string::String;
    #[cfg(feature="no_std")]
                #[cfg(test)] pub use self::alloc::string::ToString;
    #[cfg(feature="no_std")] pub use self::alloc::collections::VecDeque;
    #[cfg(feature="no_std")]
                #[cfg(test)] pub use self::alloc::format;
    #[cfg(feature="no_std")] pub use self::alloc::rc::{Rc, Weak};
    #[cfg(feature="no_std")]
                #[cfg(test)] pub use self::alloc::vec;
    #[cfg(feature="no_std")] pub use self::alloc::vec::Vec;
    #[cfg(feature="no_std")] pub use core::cell::{Cell, Ref, RefMut, RefCell};
    #[cfg(feature="no_std")] pub use core::cmp::Ordering::{self, *};
    #[cfg(feature="no_std")] pub use core::fmt::{self, Debug, Display, Formatter};
    #[cfg(feature="no_std")] pub use core::hash::{Hasher, Hash};
    #[cfg(feature="no_std")] pub use core::iter::{Iterator, FromIterator, IntoIterator, FusedIterator};
    #[cfg(feature="no_std")] pub use core::marker::{PhantomData, Unpin};
    #[cfg(feature="no_std")] pub use core::mem::{self, forget, transmute, MaybeUninit};
    #[cfg(feature="no_std")] pub use core::ops::{Add, AddAssign, Deref, DerefMut, Div, Neg, Sub, SubAssign};
    #[cfg(feature="no_std")] pub use core::pin::Pin;
    #[cfg(feature="no_std")] pub use core::ptr::{self, NonNull, null, null_mut};
}

#[macro_use]
mod macros;

pub mod tuple;
pub use tuple::{TupleForest, TupleTree};

pub mod bfs;

pub mod size;
pub use size::Size;

pub mod tree;
pub use tree::Tree;

pub mod forest;
pub use forest::Forest;

pub mod node;
pub use node::Node;
pub(crate) use node::Data;

pub(crate) mod node_vec;
pub(crate) use node_vec::NodeVec;

pub mod iter;
pub use iter::{Iter, IterMut};
pub(crate) use iter::CountedRawIter;

pub mod into_iter;
pub use into_iter::IntoIter;

pub mod heap;

pub mod walk;
pub use walk::{TreeWalk, ForestWalk};

pub mod notation;
pub use notation::{tr, fr};

pub mod iter_rc;
pub use iter_rc::IterRc;

pub mod rc;
pub use rc::{RcNode, WeakNode};

pub(crate) mod bfs_impls;
