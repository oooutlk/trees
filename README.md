This project provides various implementations of trees serving for general purpose. 

# Features

- Traversal

  Each node in a tree provides standard forward iterators for visiting its children nodes. Tree traversal can be done via using these iterators in recursive function calls.

  Depth-first search and breadth-first iterators are also provided.


- Operation

  The methods for adding or removing child tree at the front/back of a node’s children list are guaranteed constant time. Accessing, inserting or removing nodes in any position are linear time.

  All public interfaces are safe.


- Notation

  A compact notation of tree construction has been developed by overloading sub and div operators. It makes complex tree composition look like literal expression, reducing a bit of syntax noise. See the example section for more.

# Implementations

Currently this library provides only two slightly different trees, both implemented in raw-pointer-linked nodes.

- `linked::singly`
  Two pointers per node. Hold no size infomation. Note that constant time `pop_back` is not supported.

- `linked::singly`
  Four pointers plus two `u32`s per node. Hold children count and node count.

# Prominent types

Tree, Forest and Node are the big three types in this library. 

- Tree is a collection of owned nodes in hierarchical structure, with one top-level node named root.

- Forest is similar to Tree, except that it has no root node.

- Node is the underlying storage type and **opaque** to the library users. Instead, &Node and &mut Node are exposed.

# Examples

- notation of a literal tree

  ```rust
  let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
  ```
  
  It encodes a tree drawn as follows:
  
  .............
  .     0     .
  .   /   \   .
  .  1     4  .
  . / \   / \ .
  .2   3 5   6.
  .............

- use tree notation to reduce syntax noise, quoted from crate `reflection_derive`, [version 0.1.1](https://github.com/oooutlk/reflection/blob/master/reflection_derive/src/lib.rs#L202):

  ```rust
  quote! {
      #(
          -( ::reflection::variant( stringify!( #vnames ))
              /(
                  #(
                      -( ::reflection::field(
                              #fnames,
                              <#ftypes1 as ::reflection::Reflection>::ty(),
                              <#ftypes2 as ::reflection::Reflection>::name(),
                              Some( <#ftypes3 as ::reflection::Reflection>::members )))
                  )*
              )
          )
      )*
  }
  ```

  The starting of tree operations are denoted by `-(` and `/(` which are humble enough to let the reader focusing on the data part.

- use iterators if the tree travesal is a "driving wheel"( you can iterate over the tree on your own ).

  ```rust
  use trees::{tr,Node};
  use std::fmt::Display;
                                                                   
  let tree = tr(0)
      /( tr(1) /tr(2)/tr(3) )
      /( tr(4) /tr(5)/tr(6) );
                                                                   
  fn tree_to_string<T:Display>( node: &Node<T> ) -> String {
      if node.is_leaf() {
          node.data.to_string()
      } else {
          format!( "{}( {})", node.data, 
              node.iter().fold( String::new(),
                  |s,c| s + &tree_to_string(c) + &" " ))
      }
  }
                                                                   
  assert_eq!( tree_to_string( &tree ), "0( 1( 2 3 ) 4( 5 6 ) )" );
  ```

- use `TreeWalk` when the tree travesal is a "driven wheel"( driven by other library ). Quoted from crate `tsv`, [version 0.1.0](https://github.com/oooutlk/tsv/blob/master/src/de.rs#L542):

  ```rust
      fn next_value_seed<V:DeserializeSeed<'de>>( &mut self, seed: V ) -> Result<V::Value> {
          let result = self.next_element_in_row( seed )?;
          self.de.next_column();
          self.de.row += 1;
          self.de.pop_stack(); // finish key-value pair
          self.de.next_column();
          self.de.columns.revisit();
          Ok( result )
      }
  ```
  The `serde` library is driving on the schema tree when (de)serializing variables. Use `TreeWalk` methods such as `next_column` and `revisit` to follow the step.

# License

Under Apache License 2.0 or MIT License, at your will.

# Quickstart

API document and quickstart here: [docs.rs]( https://docs.rs/trees/ )
