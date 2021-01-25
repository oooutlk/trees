# Traversal via iterators

Node provides iterators `iter()`/`iter_mut()` to iterate over its child nodes,
each of which provides iterators to iterate over their child nodes, and so on.

## Example

```rust,no_run
use trees::{tr, Node};
use std::fmt::Display;

let tree = tr(0)
    /( tr(1) /tr(2)/tr(3) )
    /( tr(4) /tr(5)/tr(6) );

fn tree_to_string<T:Display>( node: &Node<T> ) -> String {
    if node.has_no_child() {
        node.data.to_string()
    } else {
        format!( "{}( {})", node.data, 
        node.iter().fold( String::new(),
            |s,c| format!( "{}{} ", s, tree_to_string(c) ))
    }
}

assert_eq!( tree_to_string( &tree ), "0( 1( 2 3 ) 4( 5 6 ) )" );
```
