use crate::rust::*;

/// A struct keeping the node's children count and all its descendants count for resource management purpose.
/// Note that `u32` is utilized rather than `usize`, because 4194304K ought to be enough for anybody.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Size {
    pub degree   : u32, // count of children node
    pub node_cnt : u32, // count of all nodes, including itself and all its descendants
}

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
