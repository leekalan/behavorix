use bevy::prelude::{Component, Resource};

use crate::prelude::*;

pub mod node_tree_mut_query;
pub mod node_tree_query;
pub mod node_tree_query_mut;

#[derive(Component, Resource)]
pub struct NodeTree<'key, Key: ?Sized + PartialEq + Eq + 'static> {
    node: Box<dyn Node<Key>>,
    keys: Vec<&'key Key>,
}

impl<'key, Key: ?Sized + PartialEq + Eq + 'static> NodeTree<'key, Key> {
    pub fn new(node: Box<dyn Node<Key>>) -> Self {
        let keys = DynNode(&*node).default_keys();

        Self { node, keys }
    }

    pub fn query<'n: 'key>(&'n self) -> NodeTreeQuery<'n, 'key, Key> {
        NodeTreeQuery {
            node: &*self.node,
            keys: KeyIterator::new(&self.keys),
        }
    }

    pub fn mut_query<'n: 'key>(&'n mut self) -> NodeTreeMutQuery<'n, 'key, Key> {
        NodeTreeMutQuery {
            node: &*self.node,
            keys: KeyIteratorMut::new(&mut self.keys),
        }
    }

    pub fn query_mut<'n: 'key>(&'n mut self) -> NodeTreeQueryMut<'n, 'key, Key> {
        NodeTreeQueryMut {
            node: &mut *self.node,
            keys: KeyIteratorMut::new(&mut self.keys),
        }
    }
}

pub(crate) struct KeyIterator<'slice, 'key, Key: ?Sized + PartialEq + Eq + 'static> {
    pub keys: &'slice [&'key Key],
    pub index: usize,
}

impl<'slice, 'key, Key: ?Sized + PartialEq + Eq + 'static> Clone
    for KeyIterator<'slice, 'key, Key>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'slice, 'key, Key: ?Sized + PartialEq + Eq + 'static> Copy for KeyIterator<'slice, 'key, Key> {}

impl<'slice, 'key, Key: ?Sized + PartialEq + Eq + 'static> KeyIterator<'slice, 'key, Key> {
    pub fn new(keys: &'slice [&'key Key]) -> Self {
        Self { keys, index: 0 }
    }
}

impl<'slice, 'key, Key: ?Sized + PartialEq + Eq + 'static> Iterator
    for KeyIterator<'slice, 'key, Key>
{
    type Item = &'key Key;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.keys.len() {
            self.index += 1;
            Some(self.keys[self.index - 1])
        } else {
            None
        }
    }
}

pub(crate) struct KeyIteratorMut<'slice, 'key, Key: ?Sized + PartialEq + Eq + 'static> {
    pub keys: &'slice mut Vec<&'key Key>,
    pub index: usize,
}

impl<'slice, 'key, Key: ?Sized + PartialEq + Eq + 'static> KeyIteratorMut<'slice, 'key, Key> {
    pub fn new(keys: &'slice mut Vec<&'key Key>) -> Self {
        Self { keys, index: 0 }
    }

    pub fn as_key_iterator(&'slice self) -> KeyIterator<'slice, 'key, Key> {
        KeyIterator::new(self.keys)
    }

    pub fn next(&mut self) -> Option<&mut &'key Key> {
        if self.index < self.keys.len() {
            self.index += 1;
            Some(&mut self.keys[self.index - 1])
        } else {
            None
        }
    }
}
