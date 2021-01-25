# Shared ownership

`Tree`s, `Forest`s have exclusive ownership and borrowing of their `Node`s are
statically checked.

A `Tree` could be safely converted to `RcNode` to support shared ownership and
dynamic borrow check.

```rust,no_run
use trees::{RcNode, Tree};
let tree = Tree::<i32>::from(( 0, (1,2,3), (4,5,6), ));
let root = RcNode::from( tree );
```

An `RcNode` could be unsafely converted to `Tree` to get exclusive ownership and
static borrow check.

```rust,no_run
let tree = unsafe{ root.into_tree() };
```

`WeakNode` is the non-owning version of `RcNode`.
