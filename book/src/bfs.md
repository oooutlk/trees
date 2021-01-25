# Breath first traversal

`Tree`/`Forest`/`Node` may provide `bfs()`/`bfs_mut()`/`into_bfs()`,
which iterate all its child nodes in the manner of breadth first search. Note
that not all data structures provide all three kinds of iterators.

During breadth first search, you can get:

1. data associated with `Node`.

2. size info -- degree and node count.

## Example of BFS iteration

```rust,no_run
use trees::{Size, bfs, tr};

let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
let visits = tree.into_bfs().iter.collect::<Vec<_>>();
assert_eq!( visits, vec![
    bfs::Visit{ data: 0, size: Size{ degree: 2, descendants: 6 }},
    bfs::Visit{ data: 1, size: Size{ degree: 2, descendants: 2 }},
    bfs::Visit{ data: 4, size: Size{ degree: 2, descendants: 2 }},
    bfs::Visit{ data: 2, size: Size{ degree: 0, descendants: 0 }},
    bfs::Visit{ data: 3, size: Size{ degree: 0, descendants: 0 }},
    bfs::Visit{ data: 5, size: Size{ degree: 0, descendants: 0 }},
    bfs::Visit{ data: 6, size: Size{ degree: 0, descendants: 0 }},
]);
```

Trees can be constructed from and converted into an owning BFS iterator, making
it a bridge for conversions between trees.

## Example of collecting scattered nodes to piled ones

```rust,no_run
use trees::{Tree, tr};

let scattered_tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
let piled_tree = Tree::<i32>::from( scattered_tree.into_bfs() );
```
