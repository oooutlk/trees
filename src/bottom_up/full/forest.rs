//! `Forest` composed of disjoint `Tree`s.

use super::{Node,Tree,Iter,IterMut,OntoIter,Size};
use rust::*;

/// A nullable forest
pub struct Forest<T> {
               sub  : *mut Node<T>,
    pub(crate) size : Size,
               mark : super::heap::Phantom<T>,
}

impl<T> Forest<T> {
    /// Makes an empty `Forest`.
    #[inline] pub fn new() -> Forest<T> { Self::from( null_mut() )}

    /// Returns the number of subtrees in `Forest`.
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let forest = tr(0) - tr(1)/tr(2)/tr(3) - tr(4)/tr(5)/tr(6);
    /// assert_eq!( forest.degree(), 3 );
    /// ```
    #[inline] pub fn degree( &self ) -> usize { self.size.degree as usize }

    /// Returns the number of all subnodes in `Forest`.
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let forest = tr(0) - tr(1)/tr(2)/tr(3) - tr(4)/tr(5)/tr(6);
    /// assert_eq!( forest.node_count(), 7 );
    /// ```
    #[inline] pub fn node_count( &self ) -> usize { self.size.node_cnt as usize }

    /// Returns `true` if the `Forest` is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::{tr,fr};
    /// let mut forest = fr();
    /// assert!( forest.is_empty() );
    /// forest.push_back( tr(1) ); 
    /// assert!( !forest.is_empty() );
    /// ```
    #[inline] pub fn is_empty( &self ) -> bool { self.sub.is_null() }

    #[inline] pub(crate) fn set_parent( &mut self, parent: *mut Node<T> ) { for child in self.iter_mut() { child.sup = parent; }}

    #[inline] pub(crate) fn set_child( &mut self, node: *mut Node<T> ) { self.sub = node; }
    #[inline] pub(crate) fn from( node: *mut Node<T> ) -> Self { Forest{ sub: node, size: Size{ degree: 0, node_cnt: 0 }, mark: PhantomData } }
    #[inline] pub(crate) fn clear( &mut self ) { self.sub = null_mut(); }

    #[inline] pub(crate) unsafe fn set_sib( &mut self, prev: *mut Node<T>, next: *mut Node<T> ) {
        (*self.head()).prev  = prev;
        (*self.tail()).next = next;
    }

    #[inline] pub(crate) unsafe fn head( &self ) -> *mut Node<T> { (*self.sub).next }
    #[inline] pub(crate) fn tail( &self ) -> *mut Node<T> { self.sub }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Node<T> { (*self.head()).next }
    #[inline] pub(crate) unsafe fn new_tail( &self ) -> *mut Node<T> { (*self.tail()).prev }

    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn adopt( &mut self, begin: *mut Node<T>, end: *mut Node<T> ) {
        unsafe {
            (*self.head()).prev = begin;
            (*self.tail()).next = end;
        }
    }

    /// Returns the first child of the forest,
    /// or None if it is empty.
    pub fn first( &self ) -> Option<&Node<T>> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some( &*self.head() )}
        }
    }

    /// Returns a mutable pointer to the first child of the forest,
    /// or None if it is empty.
    pub fn first_mut( &mut self ) -> Option<&mut Node<T>> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some( &mut *self.head() )}
        }
    }

    /// Returns the last child of the forest,
    /// or None if it is empty.
    pub fn last( &self ) -> Option<&Node<T>> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some( &*self.tail() )}
        }
    }

    /// Returns a mutable pointer to the last child of the forest,
    /// or None if it is empty.
    pub fn last_mut( &mut self ) -> Option<&mut Node<T>> {
        if self.is_empty() {
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
    /// use trees::bottom_up::full::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// forest.push_front( tr(3) );
    /// assert_eq!( forest.to_string(), "( 3 1 2 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        if self.is_empty() {
            self.set_child( tree.root );
        } else { unsafe {
            tree.set_sib( self.tail(), self.head() );
            self.adopt( tree.root, tree.root );
        }}
        self.size.degree += 1;
        self.size.node_cnt += tree.size.node_cnt;
        tree.clear();
    }

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// forest.push_back( tr(3) );
    /// assert_eq!( forest.to_string(), "( 1 2 3 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        if !self.is_empty() {
            unsafe {
                tree.set_sib( self.tail(), self.head() );
                self.adopt( tree.root, tree.root );
            }
        }
        self.set_child( tree.root );
        self.size.degree += 1;
        self.size.node_cnt += tree.size.node_cnt;
        tree.clear();
    }

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// assert_eq!( forest.pop_front(), Some( tr(1) ));
    /// assert_eq!( forest.to_string(), "( 2 )" );
    /// ```
    #[inline] pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        if self.is_empty() {
            None
        } else { unsafe {
            let front = self.head();
            if self.has_only_one_child() {
                self.clear();
            } else {
                (*self.new_head()).prev = self.tail();
                (*self.tail()).next = self.new_head();
            }
            (*front).reset_parent();
            (*front).reset_sib();
            self.size.degree -= 1;
            self.size.node_cnt -= (*front).size.node_cnt;
            Some( Tree::from( front ))
        }}
    }

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// assert_eq!( forest.pop_back(), Some( tr(2) ));
    /// assert_eq!( forest.to_string(), "( 1 )" );
    /// ```
    #[inline] pub fn pop_back( &mut self ) -> Option<Tree<T>> {
        if self.is_empty() {
            None
        } else { unsafe {
            let back = self.tail();
            if self.has_only_one_child() {
                self.clear();
            } else {
                let new_tail = self.new_tail();
                (*new_tail).next = self.head();
                (*self.head()).prev = new_tail;
                self.set_child( new_tail );
            }
            (*back).reset_parent();
            (*back).reset_sib();
            self.size.degree -= 1;
            self.size.node_cnt -= (*back).size.node_cnt;
            Some( Tree::from( back ))
        }}
    }

    /// merge the forest at front
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let mut forest1 = -tr(0)-tr(1);
    /// let mut forest2 = -tr(2)-tr(3);
    /// forest1.prepend( forest2 );
    /// assert_eq!( forest1.to_string(), "( 2 3 0 1 )" );
    /// ```
    #[inline] pub fn prepend( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_empty() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.tail(), self.head() );
                self.adopt( forest.tail(), forest_head );
            }}
            self.size += forest.size;
            forest.clear();
        }
    }

    /// merge the forest at back
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let mut forest1 = -tr(0)-tr(1);
    /// let mut forest2 = -tr(2)-tr(3);
    /// forest1.append( forest2 );
    /// assert_eq!( forest1.to_string(), "( 0 1 2 3 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if !self.is_empty() { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.tail(), self.head() );
                self.adopt( forest.tail(), forest_head );
            }}
            self.set_child( forest.tail() );
            self.size += forest.size;
            forest.clear();
        }
    }

    /// Provides a forward iterator over `Forest`'s `Tree`s' root `Node`s
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let forest = -tr(1)-tr(2);
    /// let mut iter = forest.iter();
    /// assert_eq!( iter.next(), Some( tr(1).root() ));
    /// assert_eq!( iter.next(), Some( tr(2).root() ));
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline] pub fn iter<'a>( &self ) -> Iter<'a,T> {
        if self.is_empty() {
            Iter::new( null(), null(), 0 )
        } else { unsafe {
            Iter::new( self.head(), self.tail(), self.size.degree as usize )
        }}
    }

    /// Provides a forward iterator over `Forest`'s `Tree`s' root `Node`s with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::bottom_up::full::tr;
    /// let mut forest = -tr(1)-tr(2);
    /// for child in forest.iter_mut() { child.data *= 10; }
    /// assert_eq!( forest.to_string(), "( 10 20 )" );
    /// ```
    #[inline] pub fn iter_mut<'a>( &mut self ) -> IterMut<'a,T> {
        if self.is_empty() {
            IterMut::new( null_mut(), null_mut(), 0 )
        } else { unsafe {
            IterMut::new( self.head(), self.tail(), self.size.degree as usize )
        }}
    }

    /// Provide an iterator over `Forest`'s `Subnode`s for insert/remove at any position.
    /// See `Subnode`'s document for more.
    #[inline] pub fn onto_iter<'a>( &mut self ) -> OntoIter<'a,T> {
        unsafe {
            if self.is_empty() {
                OntoIter {
                    next : null_mut(), curr: null_mut(), prev: null_mut(), sub: null_mut(),
                    psub : &mut self.sub as *mut *mut Node<T>, psize : &mut self.size as *mut Size,
                    mark : PhantomData,
                }
            } else {
                OntoIter {
                    next  : self.head(),
                    curr  : null_mut(),
                    prev  : self.sub,
                    sub   : self.sub,
                    psub  : &mut self.sub as *mut *mut Node<T>,
                    psize : &mut self.size as *mut Size,
                    mark  : PhantomData,
                }
            }
        }
    }
}

impl<T:Clone> Clone for Forest<T> {
    fn clone( &self ) -> Self {
        let mut forest = Forest::<T>::new();
        for child in self.iter() {
            forest.push_back( child.to_owned() );
        }
        forest
    }
}

impl<T> Default for Forest<T> { #[inline] fn default() -> Self { Self::new() }}

impl<T> Drop for Forest<T> {
    fn drop( &mut self ) {
        while let Some(_) = self.pop_front() {}
    }
}

pub struct IntoIter<T> {
    forest : Forest<T>,
    marker : PhantomData<Tree<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = Tree<T>;

    #[inline] fn next( &mut self ) -> Option<Tree<T>> { self.forest.pop_front() }
}

impl<T> IntoIterator for Forest<T> {
    type Item = Tree<T>;
    type IntoIter = IntoIter<T>;

    #[inline] fn into_iter( self ) -> IntoIter<T> { IntoIter{ forest: self, marker: PhantomData }}
}

impl<T> FromIterator<Tree<T>> for Forest<T> {
   fn from_iter<I:IntoIterator<Item=Tree<T>>>( iter: I ) -> Self {
        let mut iter = iter.into_iter();
        let mut children = Forest::<T>::new();
        while let Some( node ) = iter.next() {
            children.push_back( node );
        }
        children
    }
}

impl<T> Extend<Tree<T>> for Forest<T> {
    fn extend<I:IntoIterator<Item=Tree<T>>>( &mut self, iter: I ) {
        for child in iter.into_iter() {
            self.push_back( child );
        }
    }
}

impl<T:Debug> Debug for Forest<T> { fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
    if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.iter() {
                write!( f, "{:?} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:Display> Display for Forest<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_empty() {
            write!( f, "()" )
        } else {
            write!( f, "( " )?;
            for child in self.iter() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:PartialEq> PartialEq for Forest<T> {
    fn eq( &self, other: &Self ) -> bool { self.iter().eq( other.iter() )}
    fn ne( &self, other: &Self ) -> bool { self.iter().ne( other.iter() )}
}

impl<T:Eq> Eq for Forest<T> {}

impl<T:PartialOrd> PartialOrd for Forest<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        self.iter().partial_cmp( other.iter() )
    }
}

impl<T:Ord> Ord for Forest<T> {
    #[inline] fn cmp( &self, other: &Self ) -> Ordering {
        self.iter().cmp( other.iter() )
    }
}

impl<T:Hash> Hash for Forest<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        for child in self.iter() {
            child.hash( state );
        }
    }
}

unsafe impl<T:Send> Send for Forest<T> {}
unsafe impl<T:Sync> Sync for Forest<T> {}
