#![allow(unused_variables)]

pub mod dynamic_node;

use std::any::Any;

pub trait Node<Key: ?Sized + PartialEq + Eq + 'static>: Any {
    fn get_node<'n>(&'n self, key: &Key) -> Option<&'n dyn Node<Key>> {
        None
    }
    fn get_node_mut<'n>(&'n mut self, key: &Key) -> Option<&'n mut dyn Node<Key>> {
        None
    }
    fn list_nodes_and_keys<'n>(
        &'n self,
    ) -> Box<dyn Iterator<Item = (&dyn Node<Key>, &'static Key)> + 'n> {
        let list: [(&dyn Node<_>, _); 0] = [];

        Box::new(list.into_iter())
    }

    fn is_valid_key(&self, key: &Key) -> bool {
        false
    }
    fn default_node_and_key(&self) -> Option<(&dyn Node<Key>, &'static Key)> {
        None
    }
    fn default_node(&self) -> Option<&dyn Node<Key>> {
        self.default_node_and_key().map(|(node, _)| node)
    }
    fn default_key(&self) -> Option<&Key> {
        self.default_node_and_key().map(|(_, key)| key)
    }

    fn get_default_config(&self) -> Option<&dyn Any> {
        None
    }

    fn get_config<'n>(&'n self, key: &Key) -> Option<&'n dyn Any> {
        None
    }
}
