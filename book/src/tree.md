# Tree

`Tree` is composed of one or more `Node`s. A tree is always non-empty.

## Tree root

`Tree` can be constructed from a root value of type `T`, which can be accessed
later via `root()` and `root_mut()`.

## Children

`Tree` can insert/delete child `Node`s at front/back of its children list,
which is a conceptual `Forest` and can be removed once the `abandon()` is called.

## Degree

The amount of child nodes of a tree is called tree's `degree()`.

The amount of all nodes of a tree is returned by `node_count()`.

## Breadth first search

A `Tree` may be converted to an owning iterator via `into_bfs()`, which
iterates all its nodes in the manner of breadth first search.
