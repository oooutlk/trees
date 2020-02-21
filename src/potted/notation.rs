use super::{Pot,Size};

use crate::rust::*;

/// mark trait for types to be the data type of the tree/forest.
/// The type which implements this trait should allow move or clone.
pub unsafe trait TreeData: Sized + Unpin {}

macro_rules! primitive_impls {
    ($($name:ty),*) => { $(unsafe impl TreeData for $name {})* }
}

primitive_impls! {
    bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64
}

unsafe impl TreeData for &'static str {}
unsafe impl TreeData for String {}

pub unsafe trait TupleTree where Self: Sized {
    type Data;

    unsafe fn data( &self ) -> Self::Data;
    fn descendants( &self, indirect_level: usize ) -> usize;
    fn height( &self ) -> usize;
    fn nodes( &self ) -> usize;
    unsafe fn construct_node( &self, parent: usize, height: usize, offsets: &mut [usize], pot: Pot<Self::Data> );

    fn construct_all_nodes( &self, parent: usize, mut pot: Pot<Self::Data> ) {
        let height = self.height();
        let mut offsets = Vec::with_capacity( height );
        let pot_len = pot.len();
        offsets.push( pot_len );
        if height > 1 {
            offsets.push( pot_len+1 );
            for level in 2..height {
                let offset = offsets[ level-1 ] + self.descendants( level-2 );
                offsets.push( offset );
            }
            let growed = offsets[ height-1 ] + self.descendants( height-2 ) - pot_len;
            pot.grow( growed );
        } else {
            pot.grow( 1 );
        }
        unsafe{ self.construct_node( parent, 0, offsets.as_mut_slice(), pot ); }
    }
}

unsafe impl<T:TreeData> TupleTree for T {
    type Data = T;

    unsafe fn data( &self ) -> Self { ptr::read( self )}
    fn descendants( &self, _indirect_level: usize ) -> usize { 0 }
    fn height( &self ) -> usize { 1 }
    fn nodes( &self ) -> usize { 1 }
    unsafe fn construct_node( &self, parent: usize, height: usize, offsets: &mut [usize], mut pot: Pot<Self::Data> ) {
        pot.gather( parent, offsets[ height ], self.data(), Size{ degree:0, node_cnt:1 });
        offsets[ height ] += 1;
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)*))+) => {
        $(
            unsafe impl<T,$($name),*> TupleTree for (T,$($name,)*)
                where T: TupleTree<Data=T> $(,$name: TupleTree<Data=T>)*
            {
                type Data = T;

                unsafe fn data( &self ) -> T { self.0.data() }

                fn descendants( &self, indirect_level: usize ) -> usize {
                    if indirect_level == 0 {
                        $len
                    } else {
                        0 $( + self.$n.descendants( indirect_level-1 ) )*
                    }
                }

                fn height( &self ) -> usize {
                    1 + *[ 0 $(, self.$n.height() )* ].iter().max_by( |x,y| x.cmp(y) ).unwrap()
                }

                fn nodes( &self ) -> usize {
                    [ 1 $(, self.$n.nodes() )* ].iter().sum()
                }

                #[allow(unused_variables)]
                unsafe fn construct_node( &self, parent: usize, height: usize, offsets: &mut [usize], mut pot: Pot<Self::Data> ) {
                    pot.gather( parent, offsets[ height ], self.data(), Size{ degree: $len, node_cnt: self.nodes() as u32});
                    let pos = offsets[ height ];
                    offsets[ height ] += 1;
                    $( self.$n.construct_node( pos, height+1, offsets, pot ); )*
                }
            }
        )+
    }
}

tuple_impls! {
    0 => ()
    1 => (1 T1)
    2 => (1 T1 2 T2)
    3 => (1 T1 2 T2 3 T3)
    4 => (1 T1 2 T2 3 T3 4 T4)
    5 => (1 T1 2 T2 3 T3 4 T4 5 T5)
    6 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    7 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    8 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    9 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
   10 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
   11 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
   12 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
   13 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
   14 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
   15 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
   16 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16)
   17 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17)
   18 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18)
   19 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19)
   20 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20)
   21 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21)
   22 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22)
   23 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23)
   24 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24)
   25 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25)
   26 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26)
   27 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27)
   28 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28)
   29 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28 29 T29)
   30 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28 29 T29 30 T30)
   31 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28 29 T29 30 T30 31 T31)
}

pub struct FakeRoot<T>( pub PhantomData<T> );

pub fn fr<T>() -> FakeRoot<T> { FakeRoot( PhantomData )}

pub trait TupleForest where Self: Sized {
    type Data;

    fn descendants( &self, indirect_level: usize ) -> usize;
    fn height( &self ) -> usize;
    fn nodes( &self ) -> usize;
    fn construct_all_nodes( &self, parent: usize, pot: Pot<Self::Data> );
}

macro_rules! tuple_fr_impls {
    ($($len:expr => ($($n:tt $name:ident)*))+) => {
        $(
            impl<T,$($name),*> TupleForest for (FakeRoot<T>,$($name,)*)
                where T: TupleTree<Data=T> $(,$name: TupleTree<Data=T>)*
            {
                type Data = T;

                fn descendants( &self, indirect_level: usize ) -> usize {
                    if indirect_level == 0 {
                        $len
                    } else {
                        0 $( + self.$n.descendants( indirect_level-1 ) )*
                    }
                }

                fn height( &self ) -> usize {
                    0 + *[ 0 $(, self.$n.height() )* ].iter().max_by( |x,y| x.cmp(y) ).unwrap()
                }

                fn nodes( &self ) -> usize {
                    [ 0 $(, self.$n.nodes() )* ].iter().sum()
                }

                #[allow(unused_variables)]
                fn construct_all_nodes( &self, parent: usize, mut pot: Pot<Self::Data> ) {
                    let height = self.height()+1;
                    if height > 1 {
                        let mut offsets = Vec::with_capacity( height );
                        let pot_len = pot.len();
                        offsets.push( parent );
                        offsets.push( pot_len );
                        for level in 2..height {
                            let offset = offsets[ level-1 ] + self.descendants( level-2 );
                            offsets.push( offset );
                        }
                        let growed = offsets[ height-1 ] + self.descendants( height-2 ) - pot_len;
                        pot.grow( growed );

                        $(
                            unsafe{ self.$n.construct_node( offsets[0], 1, &mut offsets, pot ); }
                        )*
                    }
                }
            }
        )+
    }
}

tuple_fr_impls! {
    0 => ()
    1 => (1 T1)
    2 => (1 T1 2 T2)
    3 => (1 T1 2 T2 3 T3)
    4 => (1 T1 2 T2 3 T3 4 T4)
    5 => (1 T1 2 T2 3 T3 4 T4 5 T5)
    6 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    7 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    8 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    9 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
   10 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
   11 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
   12 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
   13 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
   14 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
   15 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
   16 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16)
   17 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17)
   18 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18)
   19 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19)
   20 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20)
   21 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21)
   22 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22)
   23 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23)
   24 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24)
   25 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25)
   26 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26)
   27 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27)
   28 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28)
   29 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28 29 T29)
   30 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28 29 T29 30 T30)
   31 => (1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19 20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28 29 T29 30 T30 31 T31)
}
