# Node

A `Node` is associated with data of type `T`, which is accessible via
`data()`/`data_mut()` or even `pub data` field.

Node may have zero or more `Node`s as its children; zero or one node as its
`parent()`.

Node can be cloned to an owning `Tree` via `to_tree()`, with all its descendant
nodes cloned.

Node can be `detach()`-ed from its parent, making a standalone `Tree`.

## Children

The children of a `Node` can be considered as a list of `Node`s, with the
ability of inserting/deleting child nodes at its front/back: `push_front()`,
`push_back()`, `pop_front()` and `pop_back()`. The first and last child can be
accessed via `front()`/`front_mut()` and `back()`/`back_mut()`.

A forest can be merged via `prepend()` or `append()`.

Node's children can be iterated via `iter()`/`iter_mut()`/`into_iter()`.

## Siblings

Two node's are siblings if their parents are the same node.

Node can add an sibling node before/after itself via `insert_prev_sib()`/
`insert_next_sib()`.

## Degree

The amount of child nodes of a node is called node's `degree()`.
A node `has_no_child()` if its degree is 0.

The amount of all nodes( including itself and its descendants ) of a node is
returned by `node_count()`.

## Breadth first search

A `Node` may provide `bfs()`/`bfs_mut()`, which iterate all its child nodes in
the manner of breadth first search.
