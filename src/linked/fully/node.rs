//! Tree node implementation.

use super::{Tree,Forest,Iter,IterMut,OntoIter,Size};
use super::bfs::{BfsTree,Splitted,Split};
use rust::*;

pub struct Link {
    pub(crate) next   : *mut Link, // next sibling
    pub(crate) child  : *mut Link, // last child
    pub(crate) prev   : *mut Link, // previous sibling
    pub(crate) parent : *mut Link,
    pub(crate) size   : Size,
}

#[repr(C)]
pub struct Node<T> {
    pub(crate) link : Link,
    pub        data : T,
}

impl<T> Deref for Node<T> {
    type Target = Link;
    fn deref( &self ) -> &Link { &self.link }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut( &mut self ) -> &mut Link { &mut self.link }
}

impl Link {
    #[inline] pub(crate) fn set_parent( &mut self, parent: *mut Self ) { self.parent = parent; }
    #[inline] pub(crate) fn reset_parent( &mut self ) { self.parent = null_mut(); }

    #[inline] pub(crate) fn set_child( &mut self, child: *mut Self ) { self.child = child; }
    #[inline] pub(crate) fn reset_child( &mut self ) { self.set_child( null_mut() ); }
    #[inline] pub(crate) fn is_leaf( &self ) -> bool { self.child.is_null() }
    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn set_sib( &mut self, prev: *mut Self, next: *mut Self ) { self.prev = prev; self.next = next; }
    #[inline] pub(crate) fn reset_sib( &mut self ) { self.prev  = self as *mut Self; self.next = self as *mut Self; }
    #[inline] pub(crate) fn has_no_sib( &self ) -> bool { self.prev as *const Self == self as *const Self && self.next as *const Self == self as *const Self }

    #[inline] pub(crate) unsafe fn head( &self ) -> *mut Self { (*self.child).next }

    #[inline] pub(crate) fn tail( &self ) -> *mut Self { self.child }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Self { (*self.head()).next }
    #[inline] pub(crate) unsafe fn new_tail( &self ) -> *mut Self { (*self.tail()).prev  }

    #[inline] pub(crate) unsafe fn adopt( &mut self, begin: *mut Self, end: *mut Self ) { (*self.head()).prev  = begin; (*self.tail()).next = end; }

    #[inline] pub(crate) fn inc_sizes( &mut self, degree: u32, node_cnt: u32 ) {
        self.size.degree += degree;
        let mut link = self as *mut Self;
        while !link.is_null() {
            unsafe {
                (*link).size.node_cnt += node_cnt;
                link = (*link).parent;
            }
        }
    }

    #[inline] pub(crate) fn dec_sizes( &mut self, degree: u32, node_cnt: u32 ) {
        self.size.degree -= degree;
        let mut link = self as *mut Self;
        while !link.is_null() {
            unsafe {
                (*link).size.node_cnt -= node_cnt;
                link = (*link).parent;
            }
        }
    }
}

impl<T> Node<T> {
    #[inline] pub fn is_leaf( &self ) -> bool { self.link.is_leaf() }

    #[inline] pub(crate) fn plink( &mut self ) -> *mut Link { &mut self.link as *mut Link }

    /// Returns the number of subtrees in `Forest`.
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// assert_eq!( tree.degree(), 2 );
    /// ```
    #[inline] pub fn degree( &self ) -> usize { self.size.degree as usize }

    /// Returns the number of all subnodes in `Forest`.
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// assert_eq!( tree.node_count(), 7 );
    /// ```
    #[inline] pub fn node_count( &self ) -> usize { self.size.node_cnt as usize }

    /// Returns the first child of the forest,
    /// or None if it is empty.
    pub fn first( &self ) -> Option<&Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &*( self.head() as *const Node<T> ))}
        }
    }

    /// Returns the given `Tree`'s children as a borrowed `Forest`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.forest().to_string(), "( 1 2 )" );
    /// ```
    #[inline] pub fn forest( &self ) -> &Forest<T> {
        unsafe{ &*( &self.link as *const Link as *const Forest<T> )}
    }

    /// Returns the given `Tree`'s children as a mutable borrowed `Forest`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for child in tree.forest_mut().iter_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn forest_mut( &mut self ) -> &mut Forest<T> {
        unsafe{ &mut *( self.plink() as *mut Forest<T> )}
    }

    /// Returns a mutable pointer to the first child of the forest,
    /// or None if it is empty.
    pub fn first_mut( &mut self ) -> Option<&mut Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &mut *( self.head() as *mut Node<T> ))}
        }
    }

    /// Returns the last child of the forest,
    /// or None if it is empty.
    pub fn last( &self ) -> Option<&Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &*( self.tail() as *const Node<T> ))}
        }
    }

    /// Returns a mutable pointer to the last child of the forest,
    /// or None if it is empty.
    pub fn last_mut( &mut self ) -> Option<&mut Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &mut *( self.tail() as *mut Node<T> ))}
        }
    }

    /// Returns the parent node of this node,
    /// or None if it is a root node.
    pub fn parent( &self ) -> Option<&Node<T>> {
        if self.parent.is_null() {
            None
        } else { unsafe {
            Some( &*( self.parent as *mut Node<T> ))
        }}
    }

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0);
    /// tree.push_front( tr(1) );
    /// assert_eq!( tree.to_string(), "0( 1 )" );
    /// tree.push_front( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 2 1 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        unsafe {
            tree.set_parent( self.plink() );
            let tree_root = tree.root_mut().plink();
            if self.is_leaf() {
                self.set_child( tree_root );
            } else {
                tree.set_sib( self.tail(), self.head() );
                self.adopt( tree_root, tree_root );
            }
        }
        self.inc_sizes( 1, tree.root().size.node_cnt );
        tree.clear();
    }

    /// Add the tree as the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0);
    /// tree.push_back( tr(1) );
    /// assert_eq!( tree.to_string(), "0( 1 )" );
    /// tree.push_back( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        unsafe {
            tree.set_parent( self.plink() );
            let tree_root = tree.root_mut().plink();
            if !self.is_leaf() {
                tree.root_mut().set_sib( self.tail(), self.head() );
                self.adopt( tree_root, tree_root );
            }
            self.set_child( tree_root );
        }
        self.inc_sizes( 1, tree.root().size.node_cnt );
        tree.clear();
    }

    /// Remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.pop_front(), Some( tr(1) ));
    /// assert_eq!( tree.to_string(), "0( 2 )" );
    /// assert_eq!( tree.pop_front(), Some( tr(2) ));
    /// assert_eq!( tree.to_string(), "0" );
    /// ```
    #[inline] pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        if self.is_leaf() {
            None
        } else { unsafe {
            let front = self.head();
            if self.has_only_one_child() {
                self.reset_child();
            } else {
                (*self.new_head()).prev = self.tail();
                (*self.tail()).next     = self.new_head();
            }
            (*front).reset_parent();
            (*front).reset_sib();
            self.dec_sizes( 1, (*front).size.node_cnt );
            Some( Tree::from( front ))
        }}

    }

    /// Remove and return the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.pop_back(), Some( tr(2) ));
    /// assert_eq!( tree.to_string(), "0( 1 )" );
    /// assert_eq!( tree.pop_back(), Some( tr(1) ));
    /// assert_eq!( tree.to_string(), "0" );
    /// ```
    #[inline] pub fn pop_back( &mut self ) -> Option<Tree<T>> {
        if self.is_leaf() {
            None
        } else { unsafe {
            let back = self.tail();
            if self.has_only_one_child() {
                self.reset_child();
            } else {
                let new_tail = self.new_tail();
                (*new_tail).next = self.head();
                (*self.head()).prev = new_tail;
                self.set_child( new_tail );
            }
            (*back).reset_parent();
            (*back).reset_sib();
            self.dec_sizes( 1, (*back).size.node_cnt );
            Some( Tree::from( back ))
        }}
    }

    /// Add all the forest's trees at front of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0);
    /// tree.prepend( -tr(1)-tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// tree.prepend( -tr(3)-tr(4) );
    /// assert_eq!( tree.to_string(), "0( 3 4 1 2 )" );
    /// ```
    #[inline] pub fn prepend( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            forest.set_parent( self.plink() );
            if self.is_leaf() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.tail(), self.head() );
                self.adopt( forest.tail(), forest_head );
            }}
            self.inc_sizes( forest.size.degree, forest.size.node_cnt );
            forest.clear();
        }
    }

    /// Add all the forest's trees at back of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0);
    /// tree.append( -tr(1)-tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// tree.append( -tr(3)-tr(4) );
    /// assert_eq!( tree.to_string(), "0( 1 2 3 4 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            forest.set_parent( self.plink() );
            if self.is_leaf() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.tail(), self.head() );
                self.adopt( forest.tail(), forest_head );
                self.set_child( forest.tail() );
            }}
            self.inc_sizes( forest.size.degree, forest.size.node_cnt );
            forest.clear();
        }
    }

    /// Provides a forward iterator over child `Node`s
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    ///
    /// let tree = tr(0);
    /// assert_eq!( tree.iter().next(), None );
    ///
    /// let tree = tr(0) /tr(1)/tr(2);
    /// let mut iter = tree.iter();
    /// assert_eq!( iter.next(), Some( tr(1).root() ));
    /// assert_eq!( iter.next(), Some( tr(2).root() ));
    /// assert_eq!( iter.next(), None );
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline] pub fn iter<'a, 's:'a>( &'s self ) -> Iter<'a,T> {
        if self.is_leaf() {
            Iter::new( null(), null(), 0 )
        } else { unsafe {
            Iter::new( self.head(), self.tail(), self.size.degree as usize )
        }}
    }

    #[deprecated( since="0.2.0", note="please use `iter` instead" )]
    #[inline] pub fn children<'a, 's:'a>( &'s self ) -> Iter<'a,T> {
        if self.is_leaf() {
            Iter::new( null(), null(), 0 )
        } else { unsafe {
            Iter::new( self.head(), self.tail(), self.size.degree as usize )
        }}
    }

    /// Provides a forward iterator over child `Node`s with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    ///
    /// let mut tree = tr(0);
    /// assert_eq!( tree.iter_mut().next(), None );
    ///
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for child in tree.iter_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn iter_mut<'a, 's:'a>( &'s mut self ) -> IterMut<'a,T> {
        if self.is_leaf() {
            IterMut::new( null_mut(), null_mut(), 0 )
        } else { unsafe {
            IterMut::new( self.head(), self.tail(), self.size.degree as usize )
        }}
    }

   #[deprecated( since="0.2.0", note="please use `iter_mut` instead" )]
     #[inline] pub fn children_mut<'a, 's:'a>( &'s mut self ) -> IterMut<'a,T> {
        if self.is_leaf() {
            IterMut::new( null_mut(), null_mut(), 0 )
        } else { unsafe {
            IterMut::new( self.head(), self.tail(), self.size.degree as usize )
        }}
    }

    /// Provide an iterator over `Node`'s `Subnode`s for insert/remove at any position.
    /// See `Subnode`'s document for more.
    #[inline] pub fn onto_iter<'a, 's:'a>( &'s mut self ) -> OntoIter<'a,T> {
        unsafe {
            if self.is_leaf() {
                OntoIter {
                    next: null_mut(), curr: null_mut(), prev: null_mut(), child: null_mut(),
                    parent : self.plink(),
                    mark: PhantomData,
                }
            } else {
                OntoIter {
                    next   : self.head(),
                    curr   : null_mut(),
                    prev   : self.child,
                    child  : self.child,
                    parent : self.plink(),
                    mark   : PhantomData,
                }
            }
        }
    }

    /// Provides a forward iterator in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::linked::fully::tr;
    ///
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let visits = tree.root().bfs().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: &0, size: Size{ degree: 2, node_cnt: 7 }},
    ///     bfs::Visit{ data: &1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn bfs( &self ) -> BfsTree<Splitted<Iter<T>>> { BfsTree::from( self, Size{ degree: 1, node_cnt: self.link.size.node_cnt })}

    /// Provides a forward iterator with mutable references in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::linked::fully::tr;
    ///
    /// let mut tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let visits = tree.root_mut().bfs_mut().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: &mut 0, size: Size{ degree: 2, node_cnt: 7 }},
    ///     bfs::Visit{ data: &mut 1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &mut 4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: &mut 2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: &mut 6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn bfs_mut( &mut self ) -> BfsTree<Splitted<IterMut<T>>> {
        let size = Size{ degree: 1, node_cnt: self.link.size.node_cnt };
        BfsTree::from( self, size )
    }
}

impl<'a, T:'a> Split for &'a Node<T> {
    type Item = &'a T;
    type Iter = Iter<'a,T>;

    fn split( self ) -> ( &'a T, Iter<'a,T>, u32 ) {
        ( &self.data, self.iter(), self.link.size.node_cnt )
    }
}

impl<'a, T:'a> Split for &'a mut Node<T> {
    type Item = &'a mut T;
    type Iter = IterMut<'a,T>;

    fn split( self ) -> ( &'a mut T, IterMut<'a,T>, u32 ) {
        let node_cnt = self.link.size.node_cnt;
        unsafe{ ( &mut *( &mut self.data as *mut T ), self.iter_mut(), node_cnt )} // borrow two mutable references at one time
    }
}

impl<'a, T:'a> IntoIterator for &'a Node<T> {
    type Item = Self;
    type IntoIter = Iter<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        let link = self as *const Node<T> as *const Link;
        Iter::new( link, link, 1 )
    }
}

impl<'a, T:'a> IntoIterator for &'a mut Node<T> {
    type Item = Self;
    type IntoIter = IterMut<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        let link = self.plink();
        IterMut::new( link, link, 1 )
    }
}

impl<T:Clone> ToOwned for Node<T> {
    type Owned = Tree<T>;
    fn to_owned( &self ) -> Self::Owned {
        let mut tree = Tree::new( self.data.clone() );
        for child in self.iter() {
            tree.push_back( child.to_owned() );
        }
        tree
    }
}

impl<T> Borrow<Forest<T>> for Tree<T> { fn borrow( &self ) -> &Forest<T> { self.forest() }}
impl<T> BorrowMut<Forest<T>> for Tree<T> { fn borrow_mut( &mut self ) -> &mut Forest<T> { self.forest_mut() }}

impl<T> Extend<Tree<T>> for Node<T> {
    fn extend<I:IntoIterator<Item=Tree<T>>>( &mut self, iter: I ) {
        for child in iter.into_iter() {
            self.push_back( child );
        }
    }
}

impl<T:Debug> Debug for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            self.data.fmt(f)
        } else {
            self.data.fmt(f)?;
            write!( f, "( " )?;
            for child in self.iter() {
                write!( f, "{:?} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:Display> Display for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            self.data.fmt(f)
        } else {
            self.data.fmt(f)?;
            write!( f, "( " )?;
            for child in self.iter() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:PartialEq> PartialEq for Node<T> {
    fn eq( &self, other: &Self ) -> bool { self.data == other.data && self.iter().eq( other.iter() )}
    fn ne( &self, other: &Self ) -> bool { self.data != other.data || self.iter().ne( other.iter() )}
}

impl<T:Eq> Eq for Node<T> {}

impl<T:PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        match self.data.partial_cmp( &other.data ) {
            None          => None,
            Some( order ) => match order {
                Less    => Some( Less ),
                Greater => Some( Greater ),
                Equal   => self.iter().partial_cmp( other.iter() ),
            },
        }
    }
}

impl<T:Ord> Ord for Node<T> {
    #[inline] fn cmp( &self, other: &Self ) -> Ordering {
        match self.data.cmp( &other.data ) {
            Less    => Less,
            Greater => Greater,
            Equal   => self.iter().cmp( other.iter() ),
        }
    }
}

impl<T:Hash> Hash for Node<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        self.data.hash( state );
        for child in self.iter() {
            child.hash( state );
        }
    }
}
