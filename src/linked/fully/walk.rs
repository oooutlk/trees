//! Walk on `Tree`/`Node` or `Forest`.

use super::{Tree,Forest,Node};

use rust::{Vec,null};

/// Distinguish between visiting a leaf node and (begin/end of) visiting a branched node.
#[derive( Copy, Clone, Debug, Eq, PartialEq )]
pub enum Visit<'a, T:'a> {
    Begin( &'a Node<T> ),
    End  ( &'a Node<T> ),
    Leaf ( &'a Node<T> ),
}

impl<'a, T:'a> Visit<'a,T> {
    /// Returns the node under visit, regardless of whether it is a leaf node or (begin/end of) visiting a branched node.
    #[inline] pub fn node( &self ) -> &Node<T> {
        match *self {
            Visit::Begin( node ) => node,
            Visit::End  ( node ) => node,
            Visit::Leaf ( node ) => node,
        }
    }
}

/// Mapping to Option<Visit>
enum VisitType { None, Begin, End, Leaf }

/// Cursor on `Node` and its siblings.
struct Nodes<T> {
    node     : *const Node<T>,
    sentinel : *const Node<T>,
}

impl<T> Nodes<T> {
    /// Only the given node will be visited.
    #[inline] fn this( node: *const Node<T> ) -> Self { Nodes{ node, sentinel: unsafe{ (*node).next as *const Node<T> }}}

    /// The given node and all its siblings will be visited.
    #[inline] fn sibs( node: *const Node<T> ) -> Self { Nodes{ node, sentinel: node }}
}

/// Control of the `Walk`'s stack.
enum Direction {
    Up,     // Current node and all its siblings and all their descendents have been visited, so go back to their parent.
    Down,   // Try to visit the first child of the current node.
    Right,  // Try to visit the next sibling of the current node.
}

/// Walk on `Node`.
struct Walk<T> {
    path       : Vec<Nodes<T>>,     // stack for keep the current node and all its ancestors.
    direction  : Direction,
    visit_type : VisitType,         // maps to Option<Visit>, needed by get().
    origin     : *const Node<T>,    // for rewind.
}

impl<T> Walk<T> {
    #[inline] fn reset( &mut self ) {
        self.path.clear();
        self.direction = Direction::Down;
        self.visit_type = VisitType::None;
    }

    #[inline] fn init_visit( &mut self ) {
        self.visit_type = 
            if let Some( nodes ) = self.path.last() {
                unsafe {
                    if (*nodes.node).is_leaf() {
                        VisitType::Leaf
                    } else {
                        VisitType::Begin
                    }
                }
            } else {
                VisitType::None
            };
    }

    #[inline] fn on_node( &mut self, node: *const Node<T> ) {
        self.reset();
        self.path.push( Nodes::this( node ));
        self.init_visit();
        self.origin = node;
    }

    #[inline] fn on_forest( &mut self, head: *const Node<T> ) {
        self.reset();
        self.path.push( Nodes::sibs( head ));
        self.init_visit();
        self.origin = head;
    }

    #[inline] fn revisit( &mut self ) {
        if !self.origin.is_null() {
            match self.visit_type {
                VisitType::None => self.path.push( Nodes::sibs( self.origin )),
                _ => (),
            }
            self.direction = Direction::Down;
            self.init_visit();
        }
    }

    /// Returns the current node in the traversal, or `None` if the traversal is completed.
    #[inline] fn get( &self ) -> Option<Visit<T>> {
        if let Some( nodes ) = self.path.last() {
            unsafe { match self.visit_type {
                VisitType::Begin => Some( Visit::Begin( &*nodes.node )),
                VisitType::End   => Some( Visit::End  ( &*nodes.node )),
                VisitType::Leaf  => Some( Visit::Leaf ( &*nodes.node )),
                VisitType::None  => None,
            }}
        } else {
            None
        }
    }

    /// Advance the cursor in the traversal.
    #[inline] fn forward( &mut self ) {
        loop {
            match self.direction {
                Direction::Up => {
                    self.path.pop();
                    if self.path.last().is_some() {
                        self.direction = Direction::Right;
                        self.visit_type = VisitType::End;
                    } else {
                        self.direction = Direction::Down;
                        self.visit_type = VisitType::None;
                    }
                    break;
                },
                Direction::Down => {
                    let new_nodes;
                    if let Some( nodes ) = self.path.last_mut() {
                        let node = unsafe{ &*nodes.node };
                        if node.is_leaf() {
                            self.direction = Direction::Right;
                            continue;
                        } else {
                            let head = unsafe{ node.head() };
                            new_nodes = Some( Nodes::sibs( head as *const Node<T> ));
                            self.visit_type = if unsafe{ (*head).is_leaf() } { VisitType::Leaf } else { VisitType::Begin };
                        }
                    } else {
                        break;
                    }
                    new_nodes.map( |nodes| self.path.push( nodes ));
                    break;
                }
                Direction::Right => {
                    if let Some( nodes ) = self.path.last_mut() {
                        nodes.node = unsafe{ (*nodes.node).next as *const Node<T> };
                        if nodes.node == nodes.sentinel {
                            self.direction = Direction::Up;
                            continue;
                        } else {
                            if unsafe{ (*nodes.node).is_leaf() } {
                                self.visit_type = VisitType::Leaf;
                            } else {
                                self.visit_type = VisitType::Begin;
                                self.direction = Direction::Down;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Advance the cursor and return the newly visited node.
    ///
    /// NOTICE: the FIRST node in the traversal can NOT be accessed via next() call.
    #[inline] fn next( &mut self ) -> Option<Visit<T>> {
        self.forward();
        self.get()
    }

    /// Set the cursor to the current node's parent and returns it, or `None` if it has no parent.
    #[inline] fn to_parent( &mut self ) -> Option<Visit<T>> {
        if self.path.last().is_some() {
            self.path.pop();
            if self.path.last().is_some() {
                self.direction = Direction::Right;
                self.visit_type = VisitType::End;
                return self.get();
            }
        }
        self.direction = Direction::Down;
        self.visit_type = VisitType::None;
        None
    }

    /// Returns the parent of current node, or `None` if it has no parent.
    #[inline] fn get_parent( &self ) -> Option<&Node<T>> {
        if self.path.len() >= 2 {
            self.path.get( self.path.len()-2 ).map( |parent| unsafe{ &*parent.node })
        } else {
            None
        }
    }

    /// Set the cursor to the current node's next `n`-th sibling and returns it, or `None` if such sibling does not exist.
    /// Returns the current node if n == 0.
    #[inline] fn to_sib( &mut self, n: usize ) -> Option<Visit<T>> {
        if let Some( nodes ) = self.path.last_mut() {
            for _ in 0..n {
                nodes.node = unsafe{ (*nodes.node).next as *const Node<T> };
                if nodes.node == nodes.sentinel {
                    self.direction = Direction::Up;
                    return None;
                }
            }
            if unsafe{ (*nodes.node).is_leaf() } {
                self.visit_type = VisitType::Leaf;
            } else {
                self.visit_type = VisitType::Begin;
                self.direction = Direction::Down;
            }
        } else {
            return None;
        }
        return self.get();
    }

    /// Set the cursor to the current node's `n`-th child and returns it, or `None` if it has no child.
    /// Notice that `n == 0` indicating the first child.
    #[inline] fn to_child( &mut self, n: usize ) -> Option<Visit<T>> {
        let new_nodes;
        if let Some( nodes ) = self.path.last_mut() {
            let node = unsafe{ &*nodes.node };
            if node.is_leaf() {
                self.direction = Direction::Right;
                return None;
            } else {
                let head = unsafe{ node.head() };
                new_nodes = Some( Nodes::sibs( head as *const Node<T> ));
                self.visit_type = if unsafe{ (*head).is_leaf() } { VisitType::Leaf } else { VisitType::Begin };
            }
        } else {
            return None;
        }
        new_nodes.map( |nodes| self.path.push( nodes ));
        self.to_sib( n )
    }
}

impl<T> Default for Walk<T> {
    #[inline] fn default() -> Self {
        Walk{ path: Vec::default(), direction: Direction::Down, visit_type: VisitType::None, origin: null() }
    }
}

/// Tree traversal
pub struct TreeWalk<T> {
    tree : Tree<T>,
    walk : Walk<T>,
}

impl<T> TreeWalk<T> {
    /// Returns the current node in the tree traversal, or `None` if the traversal is completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) / tr(1)/tr(2)/tr(3);
    /// let walk = TreeWalk::from( tree );
    /// assert_eq!( walk.get(), Some( Visit::Begin( ( tr(0)/tr(1)/tr(2)/tr(3) ).root() )));
    /// ```
    #[inline] pub fn get( &self ) -> Option<Visit<T>> { self.walk.get() }

    /// Depth first search on `TreeWalk`.
    /// Preorder or postorder at will.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let mut walk = TreeWalk::from( tree );
    /// assert_eq!( walk.get(), Some( Visit::Begin( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(2).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(3).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::End  ( (tr(1)/tr(2)/tr(3)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(4)/tr(5)/tr(6)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(5).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(6).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::End  ( (tr(4)/tr(5)/tr(6)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::End  ( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), None );
    /// walk.forward();
    /// assert_eq!( walk.get(), None );
    /// ```
    #[inline] pub fn forward( &mut self ) { self.walk.forward(); }

    /// Advance the cursor and return the newly visited node.
    ///
    /// NOTICE: the FIRST node in the traversal can NOT be accessed via next() call.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) / tr(1)/tr(2)/tr(3);
    /// let mut walk = TreeWalk::from( tree );
    /// assert_eq!( walk.next(), Some( Visit::Leaf( tr(1).root() )));
    /// assert_eq!( walk.next(), Some( Visit::Leaf( tr(2).root() )));
    /// assert_eq!( walk.next(), Some( Visit::Leaf( tr(3).root() )));
    /// assert_eq!( walk.next(), Some( Visit::End( ( tr(0)/tr(1)/tr(2)/tr(3) ).root() )));
    /// assert_eq!( walk.next(), None );
    /// assert_eq!( walk.next(), None );
    /// ```
    #[inline] pub fn next( &mut self ) -> Option<Visit<T>> { self.walk.next() }

    /// Set the cursor to the current node's parent and returns it, or `None` if it has no parent.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let mut walk = TreeWalk::from( tree );
    /// assert_eq!( walk.get(), Some( Visit::Begin( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// assert_eq!( walk.to_parent(), Some( Visit::End( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    /// ```
    #[inline] pub fn to_parent( &mut self ) -> Option<Visit<T>> { self.walk.to_parent() }

    /// Returns the parent of current node, or `None` if it has no parent.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let mut walk = TreeWalk::from( tree );
    /// assert_eq!( walk.get(), Some( Visit::Begin( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    /// assert_eq!( walk.get_parent(), None );
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// assert_eq!( walk.get_parent(), Some( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() ));
    /// ```
    #[inline] pub fn get_parent( &self ) -> Option<&Node<T>> { self.walk.get_parent() }

    /// Set the cursor to the current node's `n`-th child and returns it, or `None` if it has no child.
    /// Notice that `n == 0` indicating the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let mut walk = TreeWalk::from( tree );
    /// assert_eq!( walk.get(), Some( Visit::Begin( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    /// walk.to_child( 1 );
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(4)/tr(5)/tr(6)).root() )));
    /// ```
    #[inline] pub fn to_child( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_child(n) }

    /// Set the cursor to the current node's next `n`-th sibling and returns it, or `None` if such sibling does not exist.
    /// Returns the current node if n == 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) / tr(1)/tr(2)/tr(3);
    /// let mut walk = TreeWalk::from( tree );
    /// assert_eq!( walk.next(), Some( Visit::Leaf( tr(1).root() )));
    /// assert_eq!( walk.to_sib( 0 ), Some( Visit::Leaf( tr(1).root() )));
    /// assert_eq!( walk.to_sib( 2 ), Some( Visit::Leaf( tr(3).root() )));
    /// ```
    #[inline] pub fn to_sib( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_sib(n) }

    /// Revisit a `Node` that reached `Visit::End`.
    /// No effect on `Visit::Begin` or `Visit::Leaf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,TreeWalk};
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let mut walk = TreeWalk::from( tree );
    /// for _ in 0..3 {
    ///     for _ in 0..3 {
    ///         walk.revisit();
    ///         assert_eq!( walk.get(), Some( Visit::Begin( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    ///         walk.forward();
    ///         for _ in 0..3 {
    ///             walk.revisit();
    ///             assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    ///             walk.forward();
    ///             assert_eq!( walk.get(), Some( Visit::Leaf ( tr(2).root() )));
    ///             walk.forward();
    ///             assert_eq!( walk.get(), Some( Visit::Leaf ( tr(3).root() )));
    ///             walk.forward();
    ///             assert_eq!( walk.get(), Some( Visit::End  ( (tr(1)/tr(2)/tr(3)).root() )));
    ///         }
    ///         walk.forward();
    ///         for _ in 0..3 {
    ///             walk.revisit();
    ///             assert_eq!( walk.get(), Some( Visit::Begin( (tr(4)/tr(5)/tr(6)).root() )));
    ///             walk.forward();
    ///             assert_eq!( walk.get(), Some( Visit::Leaf ( tr(5).root() )));
    ///             walk.forward();
    ///             assert_eq!( walk.get(), Some( Visit::Leaf ( tr(6).root() )));
    ///             walk.forward();
    ///             assert_eq!( walk.get(), Some( Visit::End  ( (tr(4)/tr(5)/tr(6)).root() )));
    ///         }
    ///         walk.forward();
    ///         assert_eq!( walk.get(), Some( Visit::End  ( ( tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) ) ).root() )));
    ///     }
    ///     walk.forward();
    ///     assert_eq!( walk.get(), None );
    ///     walk.forward();
    ///     assert_eq!( walk.get(), None );
    /// }
    /// ```
    #[inline] pub fn revisit( &mut self ) { self.walk.revisit(); }
}

impl<T> From<Tree<T>> for TreeWalk<T> {
    fn from( tree: Tree<T> ) -> Self {
        let mut walk = Walk::<T>::default();
        walk.on_node( tree.root );
        TreeWalk{ tree, walk }
    }
}

impl<T> Into<Tree<T>> for TreeWalk<T> { fn into( self ) -> Tree<T> { self.tree }}

/// Forest traversal
#[derive( Default )]
pub struct ForestWalk<T> {
    forest : Forest<T>,
    walk   : Walk<T>,
}

unsafe impl<T:Send> Send for TreeWalk<T> {}
unsafe impl<T:Sync> Sync for TreeWalk<T> {}

impl<T> ForestWalk<T> {
    /// Returns the current node in the forest traversal, or `None` if the traversal is completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = -tr(1)-tr(2)-tr(3);
    /// let walk = ForestWalk::from( forest );
    /// assert_eq!( walk.get(), Some( Visit::Leaf( tr(1).root() )));
    /// ```
    #[inline] pub fn get( &self ) -> Option<Visit<T>> { self.walk.get() }

    /// Depth first search on `ForestWalk`.
    /// Preorder or postorder at will.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = - ( tr(1)/tr(2)/tr(3) ) - ( tr(4)/tr(5)/tr(6) );
    /// let mut walk = ForestWalk::from( forest );
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(2).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(3).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::End  ( (tr(1)/tr(2)/tr(3)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(4)/tr(5)/tr(6)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(5).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(6).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::End  ( (tr(4)/tr(5)/tr(6)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), None );
    /// walk.forward();
    /// assert_eq!( walk.get(), None );
    /// walk.forward();
    /// ```
    #[inline] pub fn forward( &mut self ) { self.walk.forward(); }

    /// Advance the cursor and return the newly visited node.
    ///
    /// NOTICE: the FIRST node in the traversal can NOT be accessed via next() call.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = -tr(1)-tr(2)-tr(3);
    /// let mut walk = ForestWalk::from( forest );
    /// assert_eq!( walk.next(), Some( Visit::Leaf( tr(2).root() )));
    /// assert_eq!( walk.next(), Some( Visit::Leaf( tr(3).root() )));
    /// assert_eq!( walk.next(), None );
    /// assert_eq!( walk.next(), None );
    /// ```
    #[inline] pub fn next( &mut self ) -> Option<Visit<T>> { self.walk.next() }

    /// Set the cursor to the current node's parent and returns it, or `None` if it has no parent.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = - ( tr(1)/tr(2)/tr(3) ) - ( tr(4)/tr(5)/tr(6) );
    /// let mut walk = ForestWalk::from( forest );
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(2).root() )));
    /// assert_eq!( walk.to_parent(), Some( Visit::End( (tr(1)/tr(2)/tr(3)).root() )));
    /// ```
    #[inline] pub fn to_parent( &mut self ) -> Option<Visit<T>> { self.walk.to_parent() }

    /// Returns the parent of current node, or `None` if it has no parent.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = - ( tr(1)/tr(2)/tr(3) ) - ( tr(4)/tr(5)/tr(6) );
    /// let mut walk = ForestWalk::from( forest );
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// assert_eq!( walk.get_parent(), None );
    /// walk.forward();
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(2).root() )));
    /// assert_eq!( walk.get_parent(), Some( (tr(1)/tr(2)/tr(3)).root() ));
    /// ```
    #[inline] pub fn get_parent( &self ) -> Option<&Node<T>> { self.walk.get_parent() }
 
    /// Set the cursor to the current node's `n`-th child and returns it, or `None` if it has no child.
    /// Notice that `n == 0` indicating the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = - ( tr(1)/tr(2)/tr(3) ) - ( tr(4)/tr(5)/tr(6) );
    /// let mut walk = ForestWalk::from( forest );
    /// assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    /// walk.to_child( 1 );
    /// assert_eq!( walk.get(), Some( Visit::Leaf ( tr(3).root() )));
    /// ```
    #[inline] pub fn to_child( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_child(n) }

    /// Set the cursor to the current node's next `n`-th sibling and returns it, or `None` if such sibling does not exist.
    /// Returns the current node if n == 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = -tr(1)-tr(2)-tr(3);
    /// let mut walk = ForestWalk::from( forest );
    /// assert_eq!( walk.get(), Some( Visit::Leaf( tr(1).root() )));
    /// assert_eq!( walk.to_sib( 0 ), Some( Visit::Leaf( tr(1).root() )));
    /// assert_eq!( walk.to_sib( 2 ), Some( Visit::Leaf( tr(3).root() )));
    /// ```
    #[inline] pub fn to_sib( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_sib(n) }

    /// Revisit a `Node` that reached `Visit::End`.
    /// No effect on `Visit::Begin` or `Visit::Leaf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{tr,Visit,ForestWalk};
    /// let forest = - ( tr(1)/tr(2)/tr(3) ) - ( tr(4)/tr(5)/tr(6) );
    /// let mut walk = ForestWalk::from( forest );
    /// for _ in 0..3 {
    ///     walk.revisit();
    ///     assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    ///     for _ in 0..3 {
    ///         walk.revisit();
    ///         assert_eq!( walk.get(), Some( Visit::Begin( (tr(1)/tr(2)/tr(3)).root() )));
    ///         walk.forward();
    ///         assert_eq!( walk.get(), Some( Visit::Leaf ( tr(2).root() )));
    ///         walk.forward();
    ///         assert_eq!( walk.get(), Some( Visit::Leaf ( tr(3).root() )));
    ///         walk.forward();
    ///         assert_eq!( walk.get(), Some( Visit::End  ( (tr(1)/tr(2)/tr(3)).root() )));
    ///     }
    ///     walk.forward();
    ///     for _ in 0..3 {
    ///         walk.revisit();
    ///         assert_eq!( walk.get(), Some( Visit::Begin( (tr(4)/tr(5)/tr(6)).root() )));
    ///         walk.forward();
    ///         assert_eq!( walk.get(), Some( Visit::Leaf ( tr(5).root() )));
    ///         walk.forward();
    ///         assert_eq!( walk.get(), Some( Visit::Leaf ( tr(6).root() )));
    ///         walk.forward();
    ///         assert_eq!( walk.get(), Some( Visit::End  ( (tr(4)/tr(5)/tr(6)).root() )));
    ///     }
    ///     walk.forward();
    /// }
    /// ```
    #[inline] pub fn revisit( &mut self ) { self.walk.revisit(); }
}

impl<T> From<Forest<T>> for ForestWalk<T> {
    fn from( forest: Forest<T> ) -> Self {
        let mut walk = Walk::<T>::default();
        if !forest.is_empty() {
            walk.on_forest( unsafe{ forest.head() as *const Node<T> });
        }
        ForestWalk{ forest, walk }
    }
}

impl<T> Into<Forest<T>> for ForestWalk<T> { fn into( self ) -> Forest<T> { self.forest }}

unsafe impl<T:Send> Send for ForestWalk<T> {}
unsafe impl<T:Sync> Sync for ForestWalk<T> {}
