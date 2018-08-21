//! Tree node implementation.

use super::{Tree,Forest,Iter,IterMut,OntoIter};
use rust::*;

pub struct Node<T> {
    pub(crate) sib  : *mut Node<T>, // next sibling
    pub(crate) sub  : *mut Node<T>, // last child
    pub        data : T,
}

impl<T> Node<T> {
    #[inline] pub(crate) fn set_child( &mut self, child: *mut Node<T> ) { self.sub = child; }
    #[inline] pub(crate) fn reset_child( &mut self ) { self.set_child( null_mut() ); }
    #[inline] pub fn is_leaf( &self ) -> bool { self.sub.is_null() }
    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn set_sib( &mut self, sib: *mut Self ) { self.sib = sib; }
    #[inline] pub(crate) fn reset_sib( &mut self ) { self.sib = self as *mut Self; }
    #[inline] pub(crate) fn has_no_sib( &self ) -> bool { self.sib as *const Self == self as *const Self }

    #[inline] pub(crate) unsafe fn head( &self ) -> *mut Self { (*self.sub).sib }
    #[inline] pub(crate) fn tail( &self ) -> *mut Self { self.sub }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Node<T> { (*self.head()).sib }

    #[inline] pub(crate) unsafe fn adopt( &mut self, child: *mut Node<T> ) { (*self.tail()).sib = child; }

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
    /// use trees::bottom_up::basic::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_front( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 2 1 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        unsafe {
            if self.is_leaf() {
                self.set_child( tree.root );
            } else {
                tree.set_sib( self.head() );
                self.adopt( tree.root );
            }
        }
        tree.clear();
    }

    /// add the tree as the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::basic::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_back( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        unsafe {
            if !self.is_leaf() {
                tree.set_sib( self.head() );
                self.adopt( tree.root );
            }
            self.set_child( tree.root );
        }
        tree.clear();
    }

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::basic::tr;
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
                (*self.tail()).sib = self.new_head();
            }
            (*front).reset_sib();
            Some( Tree::from( front ))
        }}

    }

    /// add all the forest's trees at front of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::basic::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.prepend( -tr(2)-tr(3) );
    /// assert_eq!( tree.to_string(), "0( 2 3 1 )" );
    /// ```
    #[inline] pub fn prepend( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_leaf() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.adopt( forest_head );
            }}
            forest.clear();
        }
    }

    /// add all the forest's trees at back of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::basic::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.append( -tr(2)-tr(3) );
    /// assert_eq!( tree.to_string(), "0( 1 2 3 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_leaf() {
                self.set_child( forest.tail() );
                forest.clear();
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.adopt( forest_head );
                self.set_child( forest.tail() );
            }}
            forest.clear();
        }
    }

    /// Provides a forward iterator over sub `Node`s
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::basic::tr;
    /// let tree = tr(0) /tr(1)/tr(2);
    /// let mut iter = tree.iter();
    /// assert_eq!( iter.next(), Some( tr(1).root() ));
    /// assert_eq!( iter.next(), Some( tr(2).root() ));
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline] pub fn iter<'a>( &self ) -> Iter<'a,T> {
        if self.is_leaf() {
            Iter::new( null(), null() )
        } else { unsafe {
            Iter::new( self.head(), self.tail() )
        }}
    }

    /// Provides a forward iterator over sub `Node`s with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::basic::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for child in tree.iter_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn iter_mut<'a>( &mut self ) -> IterMut<'a,T> {
        if self.is_leaf() {
            IterMut::new( null_mut(), null_mut() )
        } else { unsafe {
            IterMut::new( self.head(), self.tail() )
        }}
    }

    /// Provide an iterator over `Node`'s `Subnode`s for insert/remove at any position.
    /// See `Subnode`'s document for more.
    #[inline] pub fn onto_iter<'a>( &mut self ) -> OntoIter<'a,T> {
        unsafe {
            if self.is_leaf() {
                OntoIter {
                    next: null_mut(), curr: null_mut(), prev: null_mut(), sub: null_mut(),
                    psub: &mut self.sub as *mut *mut Node<T>,
                    mark: PhantomData,
                }
            } else {
                OntoIter {
                    next : self.head(),
                    curr : null_mut(),
                    prev : self.sub,
                    sub  : self.sub,
                    psub : &mut self.sub as *mut *mut Node<T>,
                    mark : PhantomData,
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
