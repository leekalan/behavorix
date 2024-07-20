use crate::prelude::*;

use std::any::Any;

pub struct NodeTreeQueryMut<'n, 'key, Key: ?Sized + PartialEq + Eq + 'static> {
    pub(super) node: &'n mut dyn Node<Key>,
    pub(super) keys: KeyIteratorMut<'n, 'key, Key>,
}

impl<'n, 'key, Key: ?Sized + PartialEq + Eq + 'static> NodeTreeQueryMut<'n, 'key, Key> {
    pub fn node(&'n self) -> &'n dyn Node<Key> {
        self.node
    }

    pub fn node_mut(&'n mut self) -> &'n mut dyn Node<Key> {
        self.node
    }

    pub fn keys(&'n self) -> impl Iterator<Item = &'key Key> + 'n {
        self.keys.as_key_iterator()
    }

    pub fn as_node_tree_query(&'n self) -> NodeTreeQuery<'n, 'key, Key> {
        NodeTreeQuery {
            node: self.node,
            keys: self.keys.as_key_iterator(),
        }
    }

    pub fn as_node_tree_mut_query(self) -> NodeTreeMutQuery<'n, 'key, Key> {
        NodeTreeMutQuery {
            node: self.node,
            keys: self.keys,
        }
    }

    pub fn find_node(self, comp_key: &Key) -> Option<Self> {
        let mut keys = self.keys;

        let mut node = self.node;

        while let Some(key) = keys.next() {
            if **key != *comp_key {
                node = match node.get_node_mut(key) {
                    Some(n) => n,
                    None => return None,
                };
            } else {
                return node.get_node_mut(comp_key).map(
                    |n| Self { node: n, keys }, //
                );
            }
        }

        None
    }

    pub fn get_default_config(&'n self) -> Option<&'n dyn Any> {
        self.node.get_default_config()
    }

    pub fn get_default_config_as<T: 'static>(&'n self) -> Option<&'n T> {
        self.node
            .get_default_config()
            .and_then(|c| c.downcast_ref())
    }

    pub fn get_config(&'n self, key: &Key) -> Option<&'n dyn Any> {
        self.node.get_config(key)
    }

    pub fn get_config_as<T: 'static>(&'n self, key: &Key) -> Option<&'n T> {
        self.node
            .get_config(key)
            .and_then(|c| c.downcast_ref::<T>())
    }
}
