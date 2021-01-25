# String representation

In current implementation of `Display`, children are separated by spaces and
grouped in the parentheses that follow their parent closely.

## Example

```text
.............
.     0     .
.   /   \   .
.  1     4  .
. / \   / \ .
.2   3 5   6.
.............
```

String representation of the tree drawn above is:

```text
0( 1( 2 3 ) 4( 5 6 ) )
```

```rust,no_run
use trees::{tr, fr};

let tree = tr(0) /( tr(1) /tr(2)/tr(3) ) /( tr(4) /tr(5)/tr(6) );
let str_repr = "0( 1( 2 3 ) 4( 5 6 ) )";
assert_eq!( tree.to_string(), str_repr );

assert_eq!( fr::<i32>().to_string(), "()" );

let forest = -( tr(1) /tr(2)/tr(3) ) -( tr(4) /tr(5)/tr(6) );
let str_repr = "( 1( 2 3 ) 4( 5 6 ) )";
assert_eq!( forest.to_string(), str_repr );
```
