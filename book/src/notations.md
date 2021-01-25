# Notations

Using `push_back()` to construct a big tree can be verbose. This library
provides some convenient notations to express trees compactly.

## Operator overloading of `-`, `/` for scatteredly stored trees

### Tree notations

```rust,no_run
use trees::tr;      // tr stands for tree

tr(0);              // a single tree node with data 0. tr(0) has no children
tr(0) /tr(1);       // tr(0) with one child tr(1)
tr(0) /tr(1)/tr(2); // tr(0) with children tr(1) and tr(2)

// tr(0) with children tr(1) and tr(4),
// tr(1) with children tr(2) and tr(3),
// tr(4) with children tr(5) and tr(6).
// Spaces and line breaks are for pretty format only.
tr(0)
    /( tr(1) /tr(2)/tr(3) )
    /( tr(4) /tr(5)/tr(6) );
```

### Forest notations

```rust,no_run
use trees::{fr, tr}; // fr stands for forest

fr::<i32>();        // an empty forest
fr() - tr(1);       // forest with one child tr(1)
- tr(1);            // same forest as above, the leading `fr()` can be omitted.
- tr(1) - tr(2);    // forest with child tr(1) and tr(2).
tr(1) - tr(2);      // same forest as above, the leading `-` can be omitted.

// forest with children tr(1) and tr(4),
// tr(1) with children tr(2) and tr(3),
// tr(4) with children tr(5) and tr(6).
-( tr(1) /tr(2)/tr(3) )
-( tr(4) /tr(5)/tr(6) );

// A tree tr(0) whose children are the forest described above.
tr(0) /(
    -( tr(1) /( -tr(2)-tr(3) ) )
    -( tr(4) /( -tr(5)-tr(6) ) )
);
```

## Tuples for contiguously stored trees

### Tree notations

```rust,no_run
let tree = trees::Tree::<i32>::from_tuple(( 0, (1,2,3), (4,5,6) ));
```

The constructed tree is equvalent to:

```rust,no_run
tr(0) /(
    -( tr(1) /( -tr(2)-tr(3) ) )
    -( tr(4) /( -tr(5)-tr(6) ) )
);
```

### Forest notations

```rust,no_run
let forest = trees::Forest::<i32>::from_tuple(( (1,2,3), (4,5,6) ));
```

The constructed forest is equivalent to:

```rust,no_run
-( tr(1) /( -tr(2)-tr(3) ) )
-( tr(4) /( -tr(5)-tr(6) ) );
```
