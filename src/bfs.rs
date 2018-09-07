/// breadth first search

use rust::*;

/// An enum for one visit in breadth first search.
#[derive(Debug,PartialEq,Eq)]
pub enum Visit<T> {
    Data( T ),      // T can be referenced, mutable referenced, or owned data
    SiblingsEnd,    // marks the end of consuming all the children of some node
    GenerationEnd,  // marks the end of consuming all the nodes of the same height in the tree
}

/// For treelike collections to split its children from its top-level data
pub trait Split<T,Item,Iter>
    where Iter: Iterator<Item=Item>
{
    fn split( self ) -> ( Option<T>, Option<Iter> );
}

// Queue elemnt composed of tree root and its children
struct Splitted<T,Iter> {
    data: Option<T>,    // None for consumed or absense( e.g. Forest has no top-level data )
    iter: Option<Iter>, // None for leaf tree node
}

impl<T,Item,Iter> Splitted<T,Iter>
    where Iter: Iterator<Item=Item>
{
    fn from<Collection>( collection: Collection ) -> Self
        where Collection: Split<T,Item,Iter>
    {
        let ( data, iter ) = collection.split();
        Self{ data, iter }
    }
}

/// An iterator in breadth-first manner
pub struct BfsIter<T,Iter> {
    queue : Vec<Splitted<T,Iter>>,
    en    : usize, // index of queue for producing more T 
    gen   : usize, // index of queue for generation separation
    de    : usize, // index of queue for consuming next T
}

impl<T,Item,Iter> BfsIter<T,Iter>
    where Iter: Iterator<Item=Item>
{
    /// Creates a `BfsIter` from a treelike collection.
    pub fn from<Collection>( collection: Collection, de: usize ) -> Self
        where Collection: Split<T,Item,Iter>
    {
        let queue = vec![ Splitted::from( collection )];
        Self{ queue, en: 0, gen: de, de }
    }

    fn next_data( &mut self ) -> Option<Visit<T>> {
        let data = self.queue[ self.de ].data.take().unwrap();
        self.de += 1;
        Some( Visit::Data( data ))
    }
}

impl<T,Item,Iter> Iterator for BfsIter<T,Iter>
    where Item: Split<T,Item,Iter>, Iter: Iterator<Item=Item>
{
    type Item = Visit<T>;

    #[inline] fn next( &mut self ) -> Option<Self::Item> {
        if self.de < self.queue.len() {
            self.next_data()
        } else if self.gen == self.en {
            if self.gen < self.queue.len() {
                self.gen = self.queue.len();
                Some( Visit::GenerationEnd )
            } else {
                None
            }
        } else if self.en < self.queue.len() {
            let tree = if let Some( ref mut iter ) = self.queue[ self.en ].iter {
                Some( iter.next() )
            } else {
                None
            };
            tree.map( |tree| { // test if any child
                tree.map( |tree| { // test if all children consumed
                    self.queue.push( Splitted::from( tree ));
                    self.next_data()
                }).unwrap_or_else( || { // all children consumed
                    self.en += 1;
                    if self.gen == self.en && self.gen < self.queue.len() {
                        self.next() // remove SiblingsEnd if followed by GenerationEnd
                    } else {
                        Some( Visit::SiblingsEnd )
                    }
                })
            }).unwrap_or_else( || { // no children
                self.en += 1;
                self.next()
            })
        } else {
           None
        }
    }
}
