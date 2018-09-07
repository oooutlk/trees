//! `Tree`/`Forest` implemented in last-child/next-sibling `Node`s, allocated on heap, with size information tracked.

pub mod tree;
pub use self::tree::Tree; 

pub mod forest;
pub use self::forest::Forest; 

pub mod node;
pub use self::node::Node;
pub(crate) use self::node::Link;

pub mod notation;
pub use self::notation::{tr,fr}; 

pub mod iter;
pub use self::iter::{Iter,IterMut};

pub mod onto_iter;
pub use self::onto_iter::{Subnode,OntoIter}; 

pub mod heap;

pub mod walk;
pub use self::walk::{Visit,TreeWalk,ForestWalk};

use super::bfs;

#[derive(Copy,Clone)]
pub struct Size {
    degree   : u32, // count of children node
    node_cnt : u32, // count of all nodes, including itself and all its descendants
}

use ::std::ops::{Add,AddAssign,Sub,SubAssign};

impl Add for Size {
    type Output = Self;
    fn add( self, rhs: Self ) -> Self { Size{ degree: self.degree+rhs.degree, node_cnt: self.node_cnt+rhs.node_cnt }}
}

impl AddAssign for Size {
    fn add_assign( &mut self, rhs: Self ) {
        *self = Size{ degree: self.degree+rhs.degree, node_cnt: self.node_cnt+rhs.node_cnt }
    }
}

impl Sub for Size {
    type Output = Self;
    fn sub( self, rhs: Self ) -> Self { Size{ degree: self.degree-rhs.degree, node_cnt: self.node_cnt-rhs.node_cnt }}
}

impl SubAssign for Size {
    fn sub_assign( &mut self, rhs: Self ) {
        *self = Size{ degree: self.degree-rhs.degree, node_cnt: self.node_cnt-rhs.node_cnt }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust::*;

    #[cfg(feature="no_std")] extern crate alloc;
    #[cfg(feature="no_std")] use self::alloc::string::ToString;

    #[test]
    fn tree_drop() {
        struct I( i32, *mut usize );

        fn i( value: i32, pcount: *mut usize ) -> Tree<I> {
            unsafe{ *pcount += 1; }
            tr( I( value, pcount ))
        }

        impl Drop for I {
            fn drop( &mut self ) {
                unsafe { *(self.1) -= 1; }
            }
        }

        impl Debug for I {
            fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
                write!( f, "{}", self.0 )
            }
        }

        let mut count:usize = 0;
        let c = &mut count as *mut usize;
        let tree = i(0,c) /( i(1,c)/i(2,c)/i(3,c) ) /( i(4,c)/i(5,c)/i(6,c) );
        assert_eq!( count, 7 );
        drop( tree );
        assert_eq!( count, 0 );
    }

    #[test]
    fn tree_equal() {
        let tr1 = tr(0) /tr(1)/tr(2);
        let tr2 = tr(0) /tr(1)/tr(2);
        let tr3 = tr(0) /tr(1)/tr(3);
        assert!( tr1 == tr2 );
        assert!( tr1 != tr3 );
    }

    #[test]
    fn notation() {
        let classic =
            tr(0) /(
                -( tr(1) /( -tr(2)-tr(3) ) )
                -( tr(4) /( -tr(5)-tr(6) ) )
            );
        let mordern = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
        assert_eq!( classic.to_string(), "0( 1( 2 3 ) 4( 5 6 ) )" );
        assert_eq!( mordern.to_string(), "0( 1( 2 3 ) 4( 5 6 ) )" );
    }

    #[test]
    fn forest_from() {
        let trees = [ tr(0), tr(1)/tr(2)/tr(3), tr(4)/tr(5)/tr(6) ];
        let forest = trees.iter().cloned().collect::<Forest<_>>();
        assert_eq!( forest, tr(0) - tr(1)/tr(2)/tr(3) - tr(4)/tr(5)/tr(6) );
    }

    #[test]
    fn forest_into() {
        let forest = tr(0) - tr(1)/tr(2)/tr(3) - tr(4)/tr(5)/tr(6);
        let mut iter = forest.into_iter();
        assert_eq!( iter.next().unwrap(), tr(0) );
        assert_eq!( iter.next().unwrap(), tr(1)/tr(2)/tr(3) );
        assert_eq!( iter.next().unwrap(), tr(4)/tr(5)/tr(6) );
        assert_eq!( iter.next(), None );
    }


    #[test]
    fn tree_extend() {
        let mut tree = tr(0) /tr(1);
        tree.extend( tr(2)-tr(3) );
        assert_eq!( tree, tr(0) /tr(1)/tr(2)/tr(3) );
    }

    #[test]
    fn forest_extend() {
        let mut forest = tr(1)-tr(2);
        forest.extend( tr(3)-tr(4) );
        assert_eq!( forest, tr(1)-tr(2)-tr(3)-tr(4) );
    }

    #[test]
    fn borrow_forest() {
        let mut tree = tr(0) /tr(1) /tr(2);
        {
            let forest: &Forest<_> = tree.borrow();
            assert_eq!( forest.to_string(), "( 1 2 )" );
        }
        {
            let forest: &mut Forest<_> = tree.borrow_mut();
            for node in forest.iter_mut() {
                node.data *= 10;
            }
        }
        assert_eq!( tree.to_string(), "0( 10 20 )" );
    }
}
