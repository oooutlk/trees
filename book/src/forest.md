# Forest

A `Forest` is composed of zero or more `Node`s. An empty forest is constructed
via `Forest::new()`.

## Children

A `Forest` can be considered as a list of `Node`s, with the ability of
inserting/deleting child nodes at its front/back: `push_front()`, `push_back()`,
`pop_front()` and `pop_back()`. The first and last child can be accessed via
`front()`/`front_mut()` and `back()`/`back_mut()`.

A forest can merge another one via `prepend()` or `append()`.

Forest's children can be iterated via `iter()`/`iter_mut()`/`into_iter()`.

## Degree

The amount of child nodes of a forest is called forest's `degree()`.
A forest `has_no_child()` if its degree is 0.

The amount of desendant nodes of a forest is returned by `node_count()`.

## Breadth first search

A `Forest` may provide `bfs()`/`bfs_mut()`/`into_bfs()`, which iterate all
its child nodes in the manner of breadth first search.
