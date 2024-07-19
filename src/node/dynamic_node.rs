use crate::prelude::*;

use std::any::Any;

pub struct DynNode<'n, Key: ?Sized + PartialEq + Eq + 'static>(pub &'n dyn Node<Key>);

impl<'n, Key: ?Sized + PartialEq + Eq + 'static> DynNode<'n, Key> {
    pub fn get_node(self, key: &Key) -> Option<&'n dyn Node<Key>> {
        self.0.get_node(key)
    }

    pub fn get_config(self, key: &Key) -> Option<&'n dyn Any> {
        self.0.get_config(key)
    }

    pub fn get_config_as<T: 'static>(self, key: &Key) -> Option<&'n T> {
        self.0.get_config(key).and_then(|c| c.downcast_ref::<T>())
    }

    pub fn default_keys(self) -> Vec<&'static Key> {
        let mut keys = Vec::with_capacity(1);

        let mut node = self.0;
        while let Some((default_node, key)) = node.default_node_and_key() {
            node = default_node;
            keys.push(key);
        }

        keys
    }
}

pub struct DynNodeMut<'n, Key: ?Sized + PartialEq + Eq + 'static>(pub &'n mut dyn Node<Key>);

impl<'n, Key: ?Sized + PartialEq + Eq + 'static> DynNodeMut<'n, Key> {
    pub fn as_dyn_node(self) -> DynNode<'n, Key> {
        DynNode(self.0)
    }

    pub fn get_node(self, key: &Key) -> Option<&'n dyn Node<Key>> {
        self.0.get_node(key)
    }

    pub fn get_node_mut(self, key: &Key) -> Option<&'n mut dyn Node<Key>> {
        self.0.get_node_mut(key)
    }

    pub fn get_config(self, key: &Key) -> Option<&'n dyn Any> {
        self.0.get_config(key)
    }

    pub fn get_config_as<T: 'static>(self, key: &Key) -> Option<&'n T> {
        self.0.get_config(key).and_then(|c| c.downcast_ref::<T>())
    }
}
