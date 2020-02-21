//! Tree node implementation.

use super::{Tree,Forest,Iter,IterMut,OntoIter,Size};
use super::bfs::{BfsTree,Splitted,Split};
use crate::rust::*;

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

    /// Returns the given `Node`'s children as a borrowed `Forest`.
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

    /// Returns the given `Node`'s children as a mutable borrowed `Forest`.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for mut child in tree.root_mut().forest_mut().iter_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn forest_mut( &mut self ) -> Pin<&mut Forest<T>> {
        unsafe{ Pin::new_unchecked( self.forest_mut_() )}
    }

    #[inline] pub(crate) fn forest_mut_( &mut self ) -> &mut Forest<T> {
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
    pub fn first_mut( &mut self ) -> Option<Pin<&mut Node<T>>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( Pin::new_unchecked( &mut *( self.head() as *mut Node<T> )))}
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
    pub fn last_mut( &mut self ) -> Option<Pin<&mut Node<T>>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( Pin::new_unchecked( &mut *( self.tail() as *mut Node<T> )))}
        }
    }

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.root_mut().push_front( tr(1) );
    /// assert_eq!( tree.to_string(), "0( 1 )" );
    /// tree.root_mut().push_front( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 2 1 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        unsafe {
            let tree_root = tree.root_mut_().plink();
            if self.is_leaf() {
                self.link.set_child( tree_root );
            } else {
                tree.link_mut().set_sib( self.head() );
                self.link.adopt( tree_root );
            }
        }
        tree.clear();
    }

    /// Add the tree as the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.root_mut().push_back( tr(1) );
    /// assert_eq!( tree.to_string(), "0( 1 )" );
    /// tree.root_mut().push_back( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        unsafe {
            let tree_root = tree.root_mut_().plink();
            if !self.is_leaf() {
                tree.link_mut().set_sib( self.head() );
                self.link.adopt( tree_root );
            }
            self.link.set_child( tree_root );
        }
        tree.clear();
    }

    /// Remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.root_mut().pop_front(), Some( tr(1) ));
    /// assert_eq!( tree.to_string(), "0( 2 )" );
    /// assert_eq!( tree.root_mut().pop_front(), Some( tr(2) ));
    /// assert_eq!( tree.to_string(), "0" );
    /// ```
    #[inline] pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        if self.is_leaf() {
            None
        } else { unsafe {
            let front = self.head();
            if self.has_only_one_child() {
                self.link.reset_child();
            } else {
                (*self.tail()).set_sib( self.new_head() );
            }
            (*front).reset_sib();
            Some( Tree::from( front ))
        }}

    }

    /// Add all the forest's trees at front of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.root_mut().prepend( -tr(1)-tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// tree.root_mut().prepend( -tr(3)-tr(4) );
    /// assert_eq!( tree.to_string(), "0( 3 4 1 2 )" );
    /// ```
    #[inline] pub fn prepend( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_leaf() {
                self.link.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.link.adopt( forest_head );
            }}
            forest.clear();
        }
    }

    /// Add all the forest's trees at back of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::singly::tr;
    /// let mut tree = tr(0);
    /// tree.root_mut().append( -tr(1)-tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// tree.root_mut().append( -tr(3)-tr(4) );
    /// assert_eq!( tree.to_string(), "0( 1 2 3 4 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_leaf() {
                self.link.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.link.adopt( forest_head );
                self.link.set_child( forest.tail() );
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
    #[inline] pub fn iter<'a, 's:'a>( &'s self ) -> Iter<'a,T> {
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
    /// assert_eq!( tree.root_mut().iter_mut().next(), None );
    ///
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for mut child in tree.root_mut().iter_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn iter_mut<'a, 's:'a>( &'s mut self ) -> IterMut<'a,T> {
        if self.is_leaf() {
            IterMut::new( null_mut(), null_mut() )
        } else { unsafe {
            IterMut::new( self.head(), self.tail() )
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
    /// use trees::{bfs,Size};
    /// use trees::linked::singly::tr;
    ///
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let visits = tree.root().bfs().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: &0, size: Size{ degree: 2, node_cnt: 0 }},
    ///     bfs::Visit{ data: &1, size: Size{ degree: 2, node_cnt: 0 }},
    ///     bfs::Visit{ data: &4, size: Size{ degree: 2, node_cnt: 0 }},
    ///     bfs::Visit{ data: &2, size: Size{ degree: 0, node_cnt: 0 }},
    ///     bfs::Visit{ data: &3, size: Size{ degree: 0, node_cnt: 0 }},
    ///     bfs::Visit{ data: &5, size: Size{ degree: 0, node_cnt: 0 }},
    ///     bfs::Visit{ data: &6, size: Size{ degree: 0, node_cnt: 0 }},
    /// ]);
    /// ```
    pub fn bfs( &self ) -> BfsTree<Splitted<Iter<T>>> { BfsTree::from( self, Size{ degree:1, node_cnt:0 })}

    /// Provides a forward iterator with mutable references in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::linked::singly::tr;
    ///
    /// let mut tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let mut root = tree.root_mut();
    /// root.bfs_mut().iter.zip( 0.. ).for_each( |(visit,nth)| (*visit.data) += 10 * nth );
    /// assert_eq!( tree, tr(0) /( tr(11)/tr(32)/tr(43) ) /( tr(24)/tr(55)/tr(66) ));
    /// ```
    pub fn bfs_mut( &mut self ) -> BfsTree<Splitted<IterMut<T>>> { BfsTree::from( unsafe{ Pin::new_unchecked( self )}, Size{ degree:1, node_cnt:0 })}
}

impl<'a, T:'a> Split for &'a Node<T> {
    type Item = &'a T;
    type Iter = Iter<'a,T>;

    fn split( self ) -> ( &'a T, Iter<'a,T>, u32 ) {
        ( &self.data, self.iter(), 0 )
    }
}

impl<'a, T:'a> Split for Pin<&'a mut Node<T>> {
    type Item = &'a mut T;
    type Iter = IterMut<'a,T>;

    fn split( self ) -> ( &'a mut T, IterMut<'a,T>, u32 ) {
        unsafe {
            let node_mut = self.get_unchecked_mut();
            let data = &mut *( &mut node_mut.data as *mut T );
            let iter = node_mut.iter_mut();
            ( data, iter, 0 ) // borrow two mutable references at one time
        }
    }
}

impl<'a, T:'a> IntoIterator for &'a Node<T> {
    type Item = Self;
    type IntoIter = Iter<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        let link = self as *const Node<T> as *const Link;
        Iter::new( link, link )
    }
}

impl<'a, T:'a> IntoIterator for Pin<&'a mut Node<T>> {
    type Item = Self;
    type IntoIter = IterMut<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        let link = unsafe{ self.get_unchecked_mut().plink() };
        IterMut::new( link, link )
    }
}

impl<T:Clone> ToOwned for Node<T> {
    type Owned = Tree<T>;
    fn to_owned( &self ) -> Self::Owned {
        let mut tree = Tree::new( self.data.clone() );
        for child in self.iter() {
            tree.root_mut_().push_back( child.to_owned() );
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

impl Debug for Link {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
            write!( f, "{{ @{:?} ↓{:?} →{:?} }}", self as *const _, self.child, self.next )
    }
}

impl<T:Debug> Debug for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            self.data.fmt(f)?;
            self.link.fmt(f)
        } else {
            self.data.fmt(f)?;
            self.link.fmt(f)?;
            write!( f, "( " )?;
            for child in self.iter() {
                child.fmt(f)?;
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
