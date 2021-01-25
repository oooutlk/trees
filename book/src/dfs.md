# Depth first traversal

`Tree<T>`/`Forest<T>` can be converted to `TreeWalk<T>`/`ForestWalk<T>` which
act like cursors for depth first traversal.

Keeping on `get()`-ing current node then `forward()`ing the cursor results in a
depth first search sequence. The two operations are combined as `next()`.

During traversal, the cursor can jump `to_parent()`/`to_child()`/`to_sib()` at
any time. To restart DFS search of a node, use `revisit()`.

## Example

```rust,no_run
use trees::{tr, Visit, TreeWalk};

let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
let mut walk = TreeWalk::from( tree );
assert_eq!( walk.get(), Some( Visit::Begin(
    ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) )
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::Begin(
    ( tr(1) /tr(2)/tr(3) )
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::Leaf(
    tr(2)
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::Leaf(
    tr(3)
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::End(
    ( tr(1) /tr(2)/tr(3) )
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::Begin(
    ( tr(4) /tr(5)/tr(6) )
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::Leaf(
    tr(5)
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::Leaf(
    tr(6)
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::End(
    ( tr(4) /tr(5)/tr(6) )
    .root() )));

walk.forward();
assert_eq!( walk.get(), Some( Visit::End(
    ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) )
    .root() )));

walk.forward();
assert_eq!( walk.get(), None );

walk.forward();
assert_eq!( walk.get(), None );
```
