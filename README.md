The "trees" project written in rust aims at:

1. expressing hierarchical data conveniently and compactly.

2. storing and manipulating tree-like data structure the simple way.

The implementation is straightforward:

1. none-intrusive nodes with child-sibling pointers.

2. children nodes, or forest, are singly-linked circular list.

This crate does not depend on libstd, and can be regarded as the nonlinear version of std::collections::LinkedList.

API document: [docs.rs]( https://docs.rs/trees/ )

# Quick start
                                                                                                                                                                                                                                            
1. `Tree` notation
                                                                                                                                                                                                                                            
```rust
use trees::tr;      // tr stands for tree
tr(0);              // A single tree node with data 0. tr(0) has no children
tr(0) /tr(1);       // tr(0) has one child tr(1)
tr(0) /tr(1)/tr(2); // tr(0) has children tr(1) and tr(2)

// tr(0) has children tr(1) and tr(4), while tr(1) has children tr(2) and tr(3), and tr(4) has children tr(5) and tr(6).
// The spaces and carriage returns are for pretty format and do not make sense.
tr(0)
    /( tr(1) /tr(2)/tr(3) )
    /( tr(4) /tr(5)/tr(6) );
```
                                                                                                                                                                                                                                            
2. `Forest` notation
                                                                                                                                                                                                                                            
```rust
use trees::{tr,fr}; // fr stands for forest
                                                                                                                                                                                                                                            
fr::<i32>();        // An empty forest
fr() - tr(1);       // forest has one child tr(1)
- tr(1);            // forest has one child tr(1). The fr() can be omitted. The Neg operator for Tree converts the tree to a forest.
- tr(1) - tr(2);    // forest has child tr(1) and tr(2)
tr(1) - tr(2);      // forest has child tr(1) and tr(2). The leading neg can be omitted.

// forest has children tr(1) and tr(4), while tr(1) has children tr(2) and tr(3), and tr(4) has children tr(5) and tr(6).
-( tr(1) /tr(2)/tr(3) )
-( tr(4) /tr(5)/tr(6) );
                                                                                                                                                                                                                                            
// A tree tr(0) whose children equal to the forest descripted above.
tr(0) /(
    -( tr(1) /( -tr(2)-tr(3) ) )
    -( tr(4) /( -tr(5)-tr(6) ) )
);
```
                                                                                                                                                                                                                                            
3. Preorder traversal
                                                                                                                                                                                                                                            
```rust
use std::string::{String,ToString};
use trees::{tr,Node};
                                                                                                                                                                                                                                            
let tree = tr(0)
    /( tr(1) /tr(2)/tr(3) )
    /( tr(4) /tr(5)/tr(6) );
                                                                                                                                                                                                                                            
fn tree_to_string<T:ToString>( node: &Node<T> ) -> String {
    if node.is_leaf() {
        node.data.to_string()
    } else {
        node.data.to_string()
            + &"( "
            + &node.children()
                .fold( String::new(),
                    |s,c| s + &tree_to_string(c) + &" " )
            + &")"
    }
}
                                                                                                                                                                                                                                            
assert_eq!( tree_to_string( &tree ), "0( 1( 2 3 ) 4( 5 6 ) )" );
```


4. String representation 

The `Debug` and `Display` trait has been implemented and are essentially the same as tree_to_tring() mentioned above.

Children are seperated by spaces and grouped in the parentheses that follow their parent closely. 

```rust
use trees::{tr,fr};

let tree = tr(0) /( tr(1) /tr(2)/tr(3) ) /( tr(4) /tr(5)/tr(6) );
let str_repr = "0( 1( 2 3 ) 4( 5 6 ) )";
assert_eq!( tree.to_string(), str_repr );
assert_eq!( format!( "{:?}", tree ), str_repr );

assert_eq!( fr::<i32>().to_string(), "()" );
assert_eq!( format!( "{:?}", fr::<i32>() ), "()" );

let forest = -( tr(1) /tr(2)/tr(3) ) -( tr(4) /tr(5)/tr(6) );
let str_repr = "( 1( 2 3 ) 4( 5 6 ) )";
assert_eq!( forest.to_string(), str_repr );
assert_eq!( format!( "{:?}", forest ), str_repr );
```
 
# Slow start
                                                                                                                                                                                                                                            
## Concepts
                                                                                                                                                                                                                                            
1. `Tree` is composed of a root `Node` and an optional `Forest` as its children. A tree can NOT be empty.
```rust
use trees::{tr,Tree,Forest};
                                                                                                                                                                                                                                            
let tree: Tree<i32> = tr(0);
let forest: Forest<i32> = -tr(1)-tr(2)-tr(3);
let mut tree = tree.adopt( forest );
let forest = tree.abandon();
```
2. `Forest` is composed of `Node`s as its children. A forest can be empty.
```rust
use trees::{tr,fr,Tree,Forest};
                                                                                                                                                                                                                                            
let mut forest: Forest<i32> = fr(); // an empty forest
forest.push_back( tr(1) );          // forest has one tree
forest.push_back( tr(2) );          // forest has two trees
```
3. `Node` is a borrowed tree, and `Tree` is an owned `Node`. All nodes in a tree can be referenced as `&Node`, but only the root node can be observed as `Tree` by the user.
```rust
use trees::{tr,Tree,Node};
use std::borrow::Borrow;
                                                                                                                                                                                                                                            
let mut tree: Tree<i32>  = tr(0) /tr(1)/tr(2)/tr(3);
{
    let root: &Node<i32> = tree.borrow();
    let first_child : &Node<i32> = tree.children().next().unwrap();
    let second_child: &Node<i32> = tree.children().nth(2).unwrap();
    let third_child : &Node<i32> = tree.children().last().unwrap();
}
let first_child: Tree<i32> = tree.pop_front().unwrap();
```
## Iterators
                                                                                                                                                                                                                                            
The children nodes of a node, or a forest, is conceptually a forward list.

1. Using `children()` to iterate over referenced child `Node`s, you can:

* read the data associated with each node.

* using `children()` to iterate over children's children, perhaps read the data associated with children's children, etc.

2. Using `children_mut()` to iterate over referenced child `Node`s, you can:

* read/write the data associated with each node, or `adopt()`, `abandon()`, `push_front()`, `pop_front()`, `push_back()` child node(s) in constant time.

* using `children_mut()` to iterate over children's children, perhaps read/write the data associated with children's children, or `adopt()`, `abandon()`, `push_front()`, `pop_front()`, `push_back()` child node(s) in constant time, etc.

3. Using `subtrees()` to iterate over `Subtree`s, you can:

* `insert_sib()`, `remove()` node(s) at given position in the children forward list in O(n) time.

* do whatever `children()` or `children_mut()` allows to do.

4. Using `Forest::<T>::into_iter()` to iterate over `Tree`s, you can:

* do whatever you want to.

## Resource management
                                                                                                                                                                                                                                            
1. `Tree`/`Forest` are implemented in extrusive manner with two extra pointers per node, and will recursively destruct all the nodes owned by the tree/forest when reaching the end of their lifetimes.
2. `Clone` for `Tree` and `Forest` makes deep copy which clones all its decendant nodes. To do copy for just one node, simplely `let cloned = trees::tr( node.data.clone() );`.
3. No bookkeeping of size information. 
                                                                                                                                                                                                                                            
## Panics
                                                                                                                                                                                                                                            
No panics unless `Clone` is involved:
* `Node::<T>::to_owned()`
* `Tree::<T>::clone()`
*  `Forest::<T>::clone()`
*  all of the operator overloading functions the operands of which contain at least one referenced type.
                                                                                                                                                                                                                                            
Panics if and only if `T::clone()` panics.
