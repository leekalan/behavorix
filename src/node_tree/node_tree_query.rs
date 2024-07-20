use crate::prelude::*;

use std::any::Any;

#[derive(Clone)]
pub struct NodeTreeQuery<'n, 'key, Key: ?Sized + PartialEq + Eq + 'static> {
    pub(super) node: &'n dyn Node<Key>,
    pub(super) keys: KeyIterator<'n, 'key, Key>,
}

impl<'n, 'key, Key: ?Sized + PartialEq + Eq + 'static> NodeTreeQuery<'n, 'key, Key> {
    pub fn node(&'n self) -> &'n dyn Node<Key> {
        self.node
    }

    pub fn keys(&self) -> impl Iterator<Item = &'key Key> + 'n {
        self.keys
    }

    pub fn find_node(&self, comp_key: &Key) -> Option<Self> {
        let mut keys = self.keys;

        let mut node = self.node;

        while let Some(key) = keys.next() {
            if *key != *comp_key {
                node = match node.get_node(key) {
                    Some(n) => n,
                    None => return None,
                };
            } else {
                return node.get_node(comp_key).map(
                    |n| Self { node: n, keys }, //
                );
            }
        }

        None
    }

    pub fn get_default_config(&self) -> Option<&'n dyn Any> {
        self.node.get_default_config()
    }

    pub fn get_default_config_as<T: 'static>(&self) -> Option<&'n T> {
        self.node
            .get_default_config()
            .and_then(|c| c.downcast_ref())
    }

    pub fn get_config(&self, key: &Key) -> Option<&'n dyn Any> {
        self.node.get_config(key)
    }

    pub fn get_config_as<T: 'static>(&self, key: &Key) -> Option<&'n T> {
        self.node
            .get_config(key)
            .and_then(|c| c.downcast_ref::<T>())
    }
}
