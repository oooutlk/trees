//! Tree node implementation.

use super::{Tree,Forest,Iter,IterMut,OntoIter};
use super::bfs;
use rust::*;

pub struct Link {
    pub(crate) next  : *mut Link, // next sibling
    pub(crate) child : *mut Link, // last child
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
    #[inline] pub(crate) fn set_child( &mut self, child: *mut Self ) { self.child = child; }
    #[inline] pub(crate) fn reset_child( &mut self ) { self.set_child( null_mut() ); }
    #[inline] pub(crate) fn is_leaf( &self ) -> bool { self.child.is_null() }
    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn set_sib( &mut self, sib: *mut Self ) { self.next = sib; }
    #[inline] pub(crate) fn reset_sib( &mut self ) { self.next = self as *mut Self; }
    #[inline] pub(crate) fn has_no_sib( &self ) -> bool { self.next as *const Self == self as *const Self }

    #[inline] pub(crate) unsafe fn head( &self ) -> *mut Self { (*self.child).next }
    #[inline] pub(crate) fn tail( &self ) -> *mut Self { self.child }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Self { (*self.head()).next }

    #[inline] pub(crate) unsafe fn adopt( &mut self, child: *mut Self ) { (*self.tail()).next = child; }
}

impl<T> Node<T> {
    #[inline] pub fn is_leaf( &self ) -> bool { self.link.is_leaf() }

    #[inline] pub(crate) fn plink( &mut self ) -> *mut Link { &mut self.link as *mut Link }

    /// Returns the given `Tree`'s children as a borrowed `Forest`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
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
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for child in tree.forest_mut().iter_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn forest_mut( &mut self ) -> &mut Forest<T> {
        unsafe{ &mut *( self.plink() as *mut Forest<T> )}
    }

    /// Returns the first child of the forest,
    /// or None if it is empty.
    pub fn first( &self ) -> Option<&Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &*( self.head() as *const Node<T> ))}
        }
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

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.push_front( tr(1) );
    /// assert_eq!( tree.to_string(), "0( 1 )" );
    /// tree.push_front( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 2 1 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        unsafe {
            let tree_root = tree.root_mut().plink();
            if self.is_leaf() {
                self.set_child( tree_root );
            } else {
                tree.set_sib( self.head() );
                self.adopt( tree_root );
            }
        }
        tree.clear();
    }

    /// add the tree as the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.push_back( tr(1) );
    /// assert_eq!( tree.to_string(), "0( 1 )" );
    /// tree.push_back( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        unsafe {
            let tree_root = tree.root_mut().plink();
            if !self.is_leaf() {
                tree.set_sib( self.head() );
                self.adopt( tree_root );
            }
            self.set_child( tree_root );
        }
        tree.clear();
    }

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
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
                (*self.tail()).set_sib( self.new_head() );
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
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.prepend( -tr(1)-tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// tree.prepend( -tr(3)-tr(4) );
    /// assert_eq!( tree.to_string(), "0( 3 4 1 2 )" );
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
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.append( -tr(1)-tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// tree.append( -tr(3)-tr(4) );
    /// assert_eq!( tree.to_string(), "0( 1 2 3 4 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_leaf() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.adopt( forest_head );
                self.set_child( forest.tail() );
            }}
            forest.clear();
        }
    }

    /// Provides a forward iterator over child `Node`s
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
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
    #[inline] pub fn iter<'a>( &self ) -> Iter<'a,T> {
        if self.is_leaf() {
            Iter::new( null(), null() )
        } else { unsafe {
            Iter::new( self.head(), self.tail() )
        }}
    }

    /// Provides a forward iterator over child `Node`s with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    ///
    /// let mut tree = tr(0);
    /// assert_eq!( tree.iter_mut().next(), None );
    ///
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
                    next: null_mut(), curr: null_mut(), prev: null_mut(), child: null_mut(),
                    parent : self.plink(),
                    mark: PhantomData,
                }
            } else {
                OntoIter {
                    next   : self.head(),
                    curr   : null_mut(),
                    prev   : self.tail(),
                    child  : self.tail(),
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
    /// use trees::bfs;
    /// use trees::linked::singly::tr;
    ///
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let visits = tree.root().bfs_iter().collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit::Data(&0),
    ///     bfs::Visit::GenerationEnd,
    ///     bfs::Visit::Data(&1),
    ///     bfs::Visit::Data(&4),
    ///     bfs::Visit::GenerationEnd,
    ///     bfs::Visit::Data(&2),
    ///     bfs::Visit::Data(&3),
    ///     bfs::Visit::SiblingsEnd,
    ///     bfs::Visit::Data(&5),
    ///     bfs::Visit::Data(&6),
    ///     bfs::Visit::GenerationEnd,
    /// ]);
    /// ```
    pub fn bfs_iter( &self ) -> bfs::BfsIter<&T,Iter<T>> { bfs::BfsIter::from( self, 0 )}

    /// Provides a forward iterator with mutable references in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bfs;
    /// use trees::linked::singly::tr;
    ///
    /// let mut tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let visits = tree.root_mut().bfs_iter_mut().collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit::Data(&mut 0),
    ///     bfs::Visit::GenerationEnd,
    ///     bfs::Visit::Data(&mut 1),
    ///     bfs::Visit::Data(&mut 4),
    ///     bfs::Visit::GenerationEnd,
    ///     bfs::Visit::Data(&mut 2),
    ///     bfs::Visit::Data(&mut 3),
    ///     bfs::Visit::SiblingsEnd,
    ///     bfs::Visit::Data(&mut 5),
    ///     bfs::Visit::Data(&mut 6),
    ///     bfs::Visit::GenerationEnd,
    /// ]);
    /// ```
    pub fn bfs_iter_mut( &mut self ) -> bfs::BfsIter<&mut T,IterMut<T>> { bfs::BfsIter::from( self, 0 )}
}

impl<'a, T:'a> bfs::Split<&'a T,&'a Node<T>,Iter<'a,T>> for &'a Node<T> {
    fn split( self ) -> ( Option<&'a T>, Option<Iter<'a,T>> ) {
        let iter = if self.is_leaf() {
            None
        } else {
            Some( self.iter() )
        };
        ( Some( &self.data ), iter )
    }
}

impl<'a, T:'a> bfs::Split<&'a mut T,&'a mut Node<T>,IterMut<'a,T>> for &'a mut Node<T> {
    fn split( self ) -> ( Option<&'a mut T>, Option<IterMut<'a,T>> ) {
        let iter = if self.is_leaf() {
            None
        } else {
            Some( self.iter_mut() )
        };
        ( Some( &mut self.data ), iter )
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

impl<T> Borrow<Forest<T>> for Tree<T> { fn borrow( &self ) -> &Forest<T> { self.forest() }}
impl<T> BorrowMut<Forest<T>> for Tree<T> { fn borrow_mut( &mut self ) -> &mut Forest<T> { self.forest_mut() }}

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
