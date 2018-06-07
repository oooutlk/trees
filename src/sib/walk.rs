//! Depth first search on `Tree`/`Node` or `Forest`.

use super::Node;

use rust::Vec;

#[derive( Debug, Eq, PartialEq )]
pub enum Visit<'a,T:'a> {
    Begin( &'a Node<T> ),
    End  ( &'a Node<T> ),
    Leaf ( &'a Node<T> ),
}

struct Nodes<'a,T:'a> {
    curr: &'a Node<T>,
    tail: *const Node<T>,
}

impl<'a,T> Nodes<'a,T> { fn new( head: &'a Node<T>, tail: *const Node<T> ) -> Self { Nodes{ curr: head, tail: tail }}}

pub struct Walk<'a,T:'a> {
    path    : Vec<Nodes<'a,T>>,
    forward : bool,
}

impl<'a,T> Walk<'a,T> {
    pub fn new( node: &'a Node<T> ) -> Self {
        let mut walk = Walk::default();
        walk.attach( node );
        walk
    }

    fn attach( &mut self, node: &'a Node<T> ) {
        self.path.push( Nodes::new( node, node as *const Node<T> ));
    }

    pub fn on( &mut self, node: &'a Node<T> ) {
        self.path.clear();
        self.attach( node );
    }
}

impl<'a,T> Default for Walk<'a,T> {
    fn default() -> Self { Walk { path: Vec::default(), forward: true }}
}

impl<'a, T:'a> Iterator for Walk<'a,T> {
    type Item = Visit<'a,T>;

    fn next( &mut self ) -> Option<Visit<'a,T>> {
        if self.forward {
            let result : Option<Visit<T>>;
            let new_nodes;
            if let Some( nodes ) = self.path.last_mut() { unsafe {
                let node = &*nodes.curr.sib;
                nodes.curr = node;
                if node.is_leaf() {
                    new_nodes = None;
                    result = Some( Visit::Leaf( node ));
                    if node as *const Node<T> == nodes.tail {
                        self.forward = false;
                    }
                } else {
                    new_nodes = Some( Nodes::new( &*node.tail(), node.tail() ));
                    result = Some( Visit::Begin( node ));
                }
            }} else {
                return None;
            }
            new_nodes.map( |nodes| self.path.push( nodes ));
            return result;
        } else {
            self.path.pop();
            if let Some( nodes ) = self.path.last_mut() {
                if nodes.curr as *const Node<T> != nodes.tail {
                    self.forward = true;
                }
                Some( Visit::End( nodes.curr ))
            } else {
                None
            }
        }
    }
}
