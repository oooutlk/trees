# Create, read, update, delete

## Tree CRUD APIs

A tree can be created via `Tree::new()` which constructs the root node only with
associated data.

```rust,no_run
use trees::Tree;
let mut tree = Tree::new(9);
```

This will construct a tree composed of only one root node with data 9.

```text
.............
.     9     .
.............
```

The root node can be accessed via `Tree::root()`/`Tree::root_mut()`.

```rust,no_run
let root = tree.root();
assert!( root.has_no_child() );
```

The associated data can be read/updated via `Tree::data()`/`Tree::data_mut()`.

```rust,no_run
assert_eq!( root.data(), &9 );

let root = tree.root_mut();
*root.data_mut() = 0;
```

The root data has been modified:

```text
.............
.     0     .
.............
```

A tree can adopt other trees as children via `Tree::push_back()`.

```rust,no_run
tree.push_back( Tree::new(1) );
tree.push_back( Tree::new(2) );
```

This will add two child nodes.

```text
.............
.     0     .
.   /   \   .
.  1     2  .
.............
```

The child nodes can be accessed via `iter()`.

```rust,no_run
let iter = tree.iter();
assert_eq!( iter.next().unwrap().data(), &1 );
assert_eq!( iter.next().unwrap().data(), &2 );
```

Specially, the first/last child can be accessed via `front()`/`back()`.

```rust,no_run
assert_eq!( tree.front().unwrap().data(), &1 );
assert_eq!( tree.back() .unwrap().data(), &2 );
```

A node can adopt other trees as children via `Node::push_back()`.

```rust,no_run
let node_1 = tree.front_mut().unwrap();
node_1.push_back( Tree::new(3) );
node_1.push_back( Tree::new(4) );
node_1.push_back( Tree::new(5) );
```

The tree will be:

```text
.............
.     0     .
.   /   \   .
.  1     2  .
. /|\       .
.3 4 5      .
.............
```

Nodes can be removed via `Node::detach()`.

```rust,no_run
let tree_4 = node_1.iter_mut().nth(1).unwrap().detach();
```

The tree will be:

```text
.............
.     0     .
.   /   \   .
.  1     2  .
. / \       .
.3   5      .
.............
```

Specially, the first/last child can be removed via `pop_front()`/`pop_back()`.

```rust,no_run
node_1.pop_front();
```

The tree will be:

```text
.............
.     0     .
.   /   \   .
.  1     2  .
.  |        .
.  5        .
.............
```

## Forest CRUD APIs

The `Forest`'s APIs are similar with `Tree`'s. The main difference is that a
`Forest` does not have root data, and may be empty.


## Node CRUD APIs

The `Node`'s APIs are similar with `Tree`'s. The main difference is that `Node`s
provided to library users are always in the form of `&Node<_>` or
`Pin<&mut Node<_>>`, which do not have ownership.
