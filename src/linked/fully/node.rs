//! Tree node implementation.

use super::{Tree,Forest,Iter,IterMut,OntoIter,Size};
use rust::*;

pub(crate) struct Link {
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

impl Link {
    #[inline] pub(crate) fn set_parent( &mut self, parent: *mut Self ) { self.parent = parent; }
    #[inline] pub(crate) fn reset_parent( &mut self ) { self.parent = null_mut(); }

    #[inline] pub(crate) fn set_child( &mut self, child: *mut Self ) { self.child = child; }
    #[inline] pub(crate) fn reset_child( &mut self ) { self.set_child( null_mut() ); }
    #[inline] pub(crate) fn has_no_child( &self ) -> bool { self.child.is_null() }
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
        self.size.node_cnt += node_cnt;
        let mut link = self.parent;
        while !link.is_null() {
            unsafe {
                (*link).size.node_cnt += node_cnt;
                link = (*link).parent;
            }
        }
    }

    #[inline] pub(crate) fn dec_sizes( &mut self, degree: u32, node_cnt: u32 ) {
        let mut link = self.parent;
        unsafe {
            if !link.is_null() {
                (*link).size.degree -= degree;
            }
        }
        while !link.is_null() {
            unsafe {
                (*link).size.node_cnt -= node_cnt;
                link = (*link).parent;
            }
        }
    }
}

impl<T> Node<T> {
    #[inline] pub(crate) fn set_parent( &mut self, parent: *mut Node<T> ) { self.link.set_parent( parent as *mut Link ); }
    #[inline] pub(crate) fn reset_parent( &mut self ) { self.link.reset_parent(); }

    #[inline] pub(crate) fn set_child( &mut self, child: *mut Node<T> ) { self.link.set_child( child as *mut Link ); }
    #[inline] pub(crate) fn reset_child( &mut self ) { self.link.reset_child(); }
    #[inline] pub fn is_leaf( &self ) -> bool { self.link.has_no_child() }
    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn set_sib( &mut self, prev: *mut Self, next: *mut Self ) { self.link.set_sib( prev as *mut Link, next as *mut Link ); }
    #[inline] pub(crate) fn reset_sib( &mut self ) { self.link.reset_sib(); }
    #[inline] pub(crate) fn has_no_sib( &self ) -> bool { self.link.has_no_sib() }

    #[inline] pub(crate) unsafe fn head( &self ) -> *mut Self { self.link.head() as *mut Self }

    #[inline] pub(crate) fn tail( &self ) -> *mut Self { self.link.tail() as *mut Self }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Node<T> { self.link.new_head() as *mut Self }
    #[inline] pub(crate) unsafe fn new_tail( &self ) -> *mut Node<T> { self.link.new_tail() as *mut Self }

    #[inline] pub(crate) unsafe fn adopt( &mut self, begin: *mut Node<T>, end: *mut Node<T> ) { self.link.adopt( begin as *mut Link, end as *mut Link )}

    #[inline] pub(crate) fn next( &self ) -> *mut Self { self.link.next as *mut Self }

    #[inline] pub(crate) fn inc_sizes( &mut self, degree: u32, node_cnt: u32 ) { self.link.inc_sizes( degree, node_cnt ); }
    #[inline] pub(crate) fn dec_sizes( &mut self, degree: u32, node_cnt: u32 ) { self.link.dec_sizes( degree, node_cnt ); }

    /// Returns the number of subtrees in `Forest`.
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// assert_eq!( tree.degree(), 2 );
    /// ```
    #[inline] pub fn degree( &self ) -> usize { self.link.size.degree as usize }

    /// Returns the number of all subnodes in `Forest`.
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// assert_eq!( tree.node_count(), 7 );
    /// ```
    #[inline] pub fn node_count( &self ) -> usize { self.link.size.node_cnt as usize }

    /// Returns the first child of the forest,
    /// or None if it is empty.
    pub fn first( &self ) -> Option<&Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &*self.head() )}
        }
    }

    /// Returns a mutable pointer to the first child of the forest,
    /// or None if it is empty.
    pub fn first_mut( &mut self ) -> Option<&mut Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &mut *self.head() )}
        }
    }

    /// Returns the last child of the forest,
    /// or None if it is empty.
    pub fn last( &self ) -> Option<&Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &*self.tail() )}
        }
    }

    /// Returns a mutable pointer to the last child of the forest,
    /// or None if it is empty.
    pub fn last_mut( &mut self ) -> Option<&mut Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &mut *self.tail() )}
        }
    }

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_front( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 2 1 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        unsafe {
            tree.set_parent( self as *mut Node<T> );
            if self.is_leaf() {
                self.set_child( tree.root );
            } else {
                tree.set_sib( self.tail(), self.head() );
                self.adopt( tree.root, tree.root );
            }
        }
        self.inc_sizes( 1, tree.root().link.size.node_cnt );
        tree.clear();
    }

    /// add the tree as the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_back( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        unsafe {
            tree.set_parent( self as *mut Node<T> );
            if !self.is_leaf() {
                tree.set_sib( self.tail(), self.head() );
                self.adopt( tree.root, tree.root );
            }
            self.set_child( tree.root );
        }
        self.inc_sizes( 1, tree.root().link.size.node_cnt );
        tree.clear();
    }

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.pop_front(), Some( tr(1) ));
    /// assert_eq!( tree.to_string(), "0( 2 )" );
    /// ```
    #[inline] pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        if self.is_leaf() {
            None
        } else { unsafe {
            let front = self.head();
            if self.has_only_one_child() {
                self.reset_child();
            } else {
                (*self.new_head()).link.prev = self.tail() as *mut Link;
                (*self.tail()).link.next = self.new_head() as *mut Link;
            }
            (*front).reset_parent();
            (*front).reset_sib();
            self.dec_sizes( 1, (*front).link.size.node_cnt );
            Some( Tree::from( front ))
        }}

    }

    /// remove and return the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.pop_back(), Some( tr(2) ));
    /// assert_eq!( tree.to_string(), "0( 1 )" );
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
                (*new_tail).link.next = self.head() as *mut Link;
                (*self.head()).link.prev = new_tail as *mut Link;
                self.set_child( new_tail );
            }
            (*back).reset_parent();
            (*back).reset_sib();
            self.dec_sizes( 1, (*back).link.size.node_cnt );
            Some( Tree::from( back ))
        }}
    }

    /// add all the forest's trees at front of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.prepend( -tr(2)-tr(3) );
    /// assert_eq!( tree.to_string(), "0( 2 3 1 )" );
    /// ```
    #[inline] pub fn prepend( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            forest.set_parent( self as *mut Node<T> as *mut Link );
            if self.is_leaf() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.tail(), self.head() );
                self.adopt( forest.tail(), forest_head );
            }}
            self.inc_sizes( forest.link.size.degree, forest.link.size.node_cnt );
            forest.clear();
        }
    }

    /// add all the forest's trees at back of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.append( -tr(2)-tr(3) );
    /// assert_eq!( tree.to_string(), "0( 1 2 3 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            forest.set_parent( self as *mut Node<T> as *mut Link );
            if self.is_leaf() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.tail(), self.head() );
                self.adopt( forest.tail(), forest_head );
                self.set_child( forest.tail() );
            }}
            self.inc_sizes( forest.link.size.degree, forest.link.size.node_cnt );
            forest.clear();
        }
    }

    /// Provides a forward iterator over sub `Node`s
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let tree = tr(0) /tr(1)/tr(2);
    /// let mut iter = tree.iter();
    /// assert_eq!( iter.next(), Some( tr(1).root() ));
    /// assert_eq!( iter.next(), Some( tr(2).root() ));
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline] pub fn iter<'a>( &self ) -> Iter<'a,T> {
        if self.is_leaf() {
            Iter::new( null(), null(), 0 )
        } else { unsafe {
            Iter::new( self.head(), self.tail(), self.link.size.degree as usize )
        }}
    }

    /// Provides a forward iterator over sub `Node`s with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for child in tree.iter_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn iter_mut<'a>( &mut self ) -> IterMut<'a,T> {
        if self.is_leaf() {
            IterMut::new( null_mut(), null_mut(), 0 )
        } else { unsafe {
            IterMut::new( self.head(), self.tail(), self.link.size.degree as usize )
        }}
    }

    /// Provide an iterator over `Node`'s `Subnode`s for insert/remove at any position.
    /// See `Subnode`'s document for more.
    #[inline] pub fn onto_iter<'a>( &mut self ) -> OntoIter<'a,T> {
        unsafe {
            if self.is_leaf() {
                OntoIter {
                    next: null_mut(), curr: null_mut(), prev: null_mut(), child: null_mut(),
                    parent : self as *mut Node<T> as *mut Link,
                    mark: PhantomData,
                }
            } else {
                OntoIter {
                    next   : self.head(),
                    curr   : null_mut(),
                    prev   : self.link.child as *mut Node<T>,
                    child  : self.link.child as *mut Node<T>,
                    parent : self as *mut Node<T> as *mut Link,
                    mark   : PhantomData,
                }
            }
        }
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
