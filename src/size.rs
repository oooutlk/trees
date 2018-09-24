#[cfg(feature="no_std")] 
use rust::*;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Size {
    pub degree   : u32, // count of children node
    pub node_cnt : u32, // count of all nodes, including itself and all its descendants
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
