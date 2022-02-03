//! # tree_sequence.rs
//!
//! Manage a sequence of trees. 
//!
//! Basically the many many children of a single root node that 
//! remains implicit.


use std::fmt;
use indextree::Arena;
use indextree::NodeId;
use crate::native::nlpsvc::node_label::*;

type TreeArena = Arena<NodeLabel>;

pub struct TreeSequence {
    first: Option<NodeId>,
    last: Option<NodeId>,
    arena: TreeArena,
}

impl TreeSequence {

    pub fn new() -> TreeSequence {
        TreeSequence {
            first: None,
            last: None,
            arena: TreeArena::new(),
        }
    }

    pub fn activate(&self, memo: CursorMemo) -> TreeCursor {
        TreeCursor::new(memo.node, &self.arena)
    }

    pub fn first(&self) -> TreeCursor {
        TreeCursor::new(self.first, &self.arena)
    }

    pub fn print(&self) {
        print_tree_sequence(self.first, &self.arena, 0);
    }

    /// Create a new atomic tree and append it to the tree sequence 
    ///
    /// This should end up being roughly equivalent to chunk() with no 
    /// child sequence.
    pub fn push_back(&mut self, lbl: NodeLabel) {
        let node = self.arena.new_node(lbl);
        match self.last {
            None => {
                // then so is self.first
                self.last = Some(node);
                self.first = self.last;
            }
            Some(last_node) => {
                last_node.insert_after(node, &mut self.arena);
                self.last = Some(node);
            }
        }
    }

    /// `end` is not included in the interval. So it could be None, 
    /// if we were working on the very tail of the tree list. 
    /// That case has to be accounted for!
    ///
    /// This should probably return a TreeCursor wrapping the new root node.
    pub fn chunk(&mut self, lbl: NodeLabel, begin: CursorMemo, end: CursorMemo) {
        // 1. Check that begin and end are not None
        // 2. Check that they are not equal
        let root: NodeId = self.arena.new_node(lbl);
        let end_id: NodeId = end.node.unwrap();
        let mut child: NodeId = begin.node.unwrap();
        let b_off = self.arena[child].data.get_span().unwrap().0;
        let mut e_off = 0;
        child.insert_before(root, &mut self.arena);
        while child != end_id {
            e_off = self.arena[child].data.get_span().unwrap().1;
            //println!("DEBUG  {:?} != {:?}", child, end_id);
            root.append(child, &mut self.arena);
            let next_opt = self.arena[root].next_sibling();
            //println!("DEBUG  next_opt = {:?}", next_opt);
            child = next_opt.unwrap();
        }
        self.arena[root].data.set_span(b_off, e_off);
    }
}



pub struct TreeCursor<'a> {
    node: Option<NodeId>,
    arena: &'a TreeArena,
}

#[derive(Debug)]
pub struct CursorMemo {
    node: Option<NodeId>,
}

/// Once you move off the edge of the tree, you can't go back, so maybe
/// this struct needs some lookahead methods as well?
/// Maybe always leave behind a copy? Instead of returning Option<NodeId>,
/// return a TreeCursor?
impl<'a> TreeCursor<'a> {

    pub fn new(node: Option<NodeId>, arena: &TreeArena) -> TreeCursor {
        TreeCursor { node: node, arena }
    }

    pub fn is_valid(&self) -> bool {
        self.node.is_some()
    }

    pub fn get(&self) -> Option<&NodeLabel> {
        match self.node {
            None => None,
            Some(node) => Some(&self.arena[node].data)
        }
    }

    pub fn to_memo(&self) -> CursorMemo {
        CursorMemo { node: self.node }
    }

    /// Move the cursor up
    ///
    /// Returns the previous value of self.node.
    /// It's based on an iterator. Imagine an iterator was sitting on the 
    /// first element of a sequence. You would want to both get that value,
    /// and increment the iterator. Otherwise you would never see that value.
    /// There could be a `get()` method or something, but it would never 
    /// work in a `for x in iter` pattern, I don't guess.
    ///
    /// This may not be the best behavior for a cursor, though.
    /// Just always remember to use cursor.up().get(), and ignore the 
    /// return value?
    pub fn up(&mut self) -> Option<NodeId> {
        match self.node.take() {
            Some(node) => {
                self.node = self.arena[node].parent();
                Some(node)
            }
            None => None
        }
    }

    /// Move the cursor to its right sibling
    pub fn next(&mut self) -> Option<NodeId> {
        match self.node.take() {
            Some(node) => {
                self.node = self.arena[node].next_sibling();
                Some(node)
            }
            None => None
        }
    }

    /// Move the cursor to its leftmost child
    pub fn first(&mut self) -> Option<NodeId> {
        match self.node.take() {
            Some(node) => {
                self.node = self.arena[node].first_child();
                Some(node)
            }
            None => None
        }
    }
}


impl<'a> fmt::Debug for TreeCursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeCursor {{ node: {:?}, arena: ... }}", self.node)
    }
}



/// Used to print a root-list when there is no single root node.
///
/// The root list is formed by a sequence of nodes connected as siblings,
/// but with no parents.
fn print_tree_sequence(node: Option<NodeId>, arena: &TreeArena, depth: i32) {
    if node.is_none() {
        return;
    }
    for t in node.unwrap().following_siblings(arena) {
        print_tree(t, arena, depth);
    }
}

/// Print a tree in outline form 
///
/// Indent tab size is hard-coded as 4.
fn print_tree(node: NodeId, arena: &TreeArena, depth: i32) {
    // print label at indent
    let indent = depth * 4;
    for _ in 0..indent {
        print!(" ");
    }
    println!("{}", arena[node].data);
    // print child list at depth + 1
    for t in node.children(arena) {
        print_tree(t, arena, depth + 1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //assert_eq!(4, add_two(2));
    }
}

