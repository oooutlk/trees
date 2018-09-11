/// breadth first search

use rust::*;

/// An enum for one visit in breadth first search.
#[derive(Debug,PartialEq,Eq)]
pub struct Visit<T> {
    pub data   : T,
    pub degree : usize,
}

pub trait Split {
    type Item;
    type Iter: ExactSizeIterator;

    fn split( self ) -> ( Self::Item, Self::Iter );
}

/// An iterator in breadth-first manner
pub struct BfsIter<Iter> {
    iters : VecDeque<Iter>,
}

impl<Treelike,Item,Iter> From<Treelike> for BfsIter<Iter>
    where Treelike : IntoIterator<Item=Item,IntoIter=Iter>
        ,     Iter : Iterator<Item=Item>
{
    fn from( treelike: Treelike ) -> Self {
        let mut iters = VecDeque::new();
        iters.push_back( treelike.into_iter() );
        BfsIter{ iters }
    }
}

impl<T,Item,Iter> Iterator for BfsIter<Iter>
    where Iter : ExactSizeIterator<Item=Item>
        , Item : Split<Iter=Iter,Item=T>
{
    type Item = Visit<T>;

    #[inline] fn next( &mut self ) -> Option<Self::Item> {
        loop {
            let next_item = 
                if let Some( ref mut iter ) = self.iters.front_mut() {
                    iter.next()
                } else {
                    return None;
                };
            if let Some( item ) = next_item {
                let ( data, iter ) = item.split();
                let degree = iter.len();
                self.iters.push_back( iter );
                return Some( Visit{ data, degree });
            } else {
                self.iters.pop_front();
            }
        }
    }
}

pub struct BfsOnTree<Iter> { pub iter: BfsIter<Iter> }

pub struct BfsOnForest<Iter> {
    pub degree : usize,
    pub iter   : BfsIter<Iter>,
}

pub enum Bfs<Iter> {
    OnTree( BfsOnTree<Iter> ),
    OnForest( BfsOnForest<Iter> ),
}

impl<Item,Iter> Bfs<Iter>
    where Iter: Iterator<Item=Item>
{
    pub fn from_tree<Treelike>( treelike: Treelike ) -> Self
        where Treelike: IntoIterator<Item=Item,IntoIter=Iter>
    {
        Bfs::OnTree( BfsOnTree{ iter: BfsIter::<Iter>::from( treelike )})
    }

    pub fn from_forest<Treelike>( degree: usize, treelike: Treelike ) -> Self
        where Treelike: IntoIterator<Item=Item,IntoIter=Iter>
    {
        Bfs::OnForest( BfsOnForest{ degree, iter: BfsIter::<Iter>::from( treelike )})
    }

    pub fn iter( self ) -> BfsIter<Iter> {
        match self {
            Bfs::OnTree(   on_tree   ) => on_tree.iter,
            Bfs::OnForest( on_forest ) => on_forest.iter,
        }
    }
}
