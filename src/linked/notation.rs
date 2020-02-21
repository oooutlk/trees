// NOTICE
// This file is the implementation of notation mod of both linked::singly's and linked::fully's.
// It is intended to be `include!()` by them, do NOT use it elsewhere.

#[inline] pub fn tr<T>( data: T ) -> Tree<T> { Tree::<T>::new( data )}

#[inline] pub fn fr<T>() -> Forest<T> { Forest::<T>::new() }

// - Tree
impl<T> Neg for Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn neg( self ) -> Forest<T> {
        let mut forest = fr();
        forest.push_back( self );
        forest
    }
}
 
// - &Tree
impl<'a,T:Clone> Neg for &'a Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn neg( self ) -> Forest<T> {
        let mut forest = fr();
        forest.push_back( self.clone() );
        forest
    }
}

// Tree - Tree
impl<T> Sub<Self> for Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Self ) -> Forest<T> {
        let mut forest = fr();
        forest.push_back( self );
        forest.push_back( rhs );
        forest
    } 
}

// Tree - &Tree
impl<'a,T:Clone> Sub<&'a Tree<T>> for Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'a Tree<T> ) -> Forest<T> {
        let mut forest = fr();
        forest.push_back( self );
        forest.push_back( rhs.clone() );
        forest
    } 
}

// &Tree - Tree
impl<'a,T:Clone> Sub<Tree<T>> for &'a Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Tree<T> ) -> Forest<T> {
        let mut forest = fr();
        forest.push_back( self.clone() );
        forest.push_back( rhs );
        forest
    } 
}

// &Tree - &Tree
impl<'a,T:Clone> Sub<Self> for &'a Tree<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Self ) -> Forest<T> {
        let mut forest = fr();
        forest.push_back( self.clone() );
        forest.push_back( rhs.clone() );
        forest
    } 
}

// Tree / Forest
impl<T> Div<Forest<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( mut self, rhs: Forest<T> ) -> Tree<T> {
        self.root_mut_().append( rhs );
        self
    } 
}

// Tree / &Forest
impl<'a,T:Clone> Div<&'a Forest<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( mut self, rhs: &'a Forest<T> ) -> Tree<T> {
        self.root_mut_().append( rhs.clone() );
        self
    } 
}

// &Tree / Forest
impl<'a,T:Clone> Div<Forest<T>> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Forest<T> ) -> Tree<T> {
        let mut tree = self.clone();
        tree.root_mut_().append( rhs );
        tree
    } 
}

// &Tree / &Forest
impl<'a,T:Clone> Div<&'a Forest<T>> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: &'a Forest<T> ) -> Tree<T> {
        let mut tree = self.clone();
        tree.root_mut_().append( rhs.clone() );
        tree
    } 
}

// Tree / Tree
impl<T> Div<Tree<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( mut self, rhs: Tree<T> ) -> Tree<T> {
        self.root_mut_().push_back( rhs );
        self
    } 
}

// Tree / &Tree
impl<'a,T:Clone> Div<&'a Tree<T>> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( mut self, rhs: &'a Tree<T> ) -> Tree<T> {
        self.root_mut_().push_back( rhs.clone() );
        self
    } 
}

// &Tree / Tree
impl<'a,T:Clone> Div<Tree<T>> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Tree<T> ) -> Tree<T> {
        let mut tree = self.clone();
        tree.root_mut_().push_back( rhs );
        tree
    } 
}

// &Tree / &Tree
impl<'a,T:Clone> Div<Self> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, rhs: Self ) -> Tree<T> {
        let mut tree = self.clone();
        tree.root_mut_().push_back( rhs.clone() );
        tree
    } 
}

// Tree / ()
impl<T> Div<()> for Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, _rhs: () ) -> Tree<T> {
        self
    } 
}

// &Tree / ()
impl<'a,T:Clone> Div<()> for &'a Tree<T> {
    type Output = Tree<T>;

    #[inline]
    fn div( self, _rhs: () ) -> Tree<T> {
        self.clone()
    } 
}

// Forest - Tree
impl<T> Sub<Tree<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( mut self, rhs: Tree<T> ) -> Self {
        self.push_back( rhs );
        self
    }
}

// Forest - &Tree
impl<'a,T:Clone> Sub<&'a Tree<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( mut self, rhs: &'a Tree<T> ) -> Self {
        self.push_back( rhs.clone() );
        self
    }
}

// &Forest - Tree
impl<'a,T:Clone> Sub<Tree<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: Tree<T> ) -> Forest<T> {
        let mut forest = self.clone();
        forest.push_back( rhs );
        forest
    }
}

// &Forest - &Tree
impl<'a,'b,T:Clone> Sub<&'b Tree<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'b Tree<T> ) -> Forest<T> {
        let mut forest = self.clone();
        forest.push_back( rhs.clone() );
        forest
    }
}

// Forest - Forest
impl<T> Sub<Forest<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( mut self, rhs: Self ) -> Self {
        self.append( rhs );
        self
    }
}

// Forest - &Forest
impl<'a,T:Clone> Sub<&'a Forest<T>> for Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( mut self, rhs: &'a Forest<T> ) -> Self {
        self.append( rhs.clone() );
        self
    }
}

// &Forest - Forest
impl<'a,T:Clone> Sub<Forest<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, mut rhs: Forest<T> ) -> Forest<T> {
        rhs.prepend( self.clone() );
        rhs
    }
}

// &Forest - &Forest
impl<'a,'b,T:Clone> Sub<&'b Forest<T>> for &'a Forest<T> {
    type Output = Forest<T>;

    #[inline]
    fn sub( self, rhs: &'b Forest<T> ) -> Forest<T> {
        let mut forest = self.clone();
        forest.append( rhs.clone() );
        forest
    }
}
