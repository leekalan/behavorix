pub mod dynamic_node;

use std::any::Any;

pub trait Node<Key: ?Sized + PartialEq + Eq + 'static>: Any {
    fn get_node<'n>(&'n self, key: &Key) -> Option<&'n dyn Node<Key>>;
    fn get_node_mut<'n>(&'n mut self, key: &Key) -> Option<&'n mut dyn Node<Key>>;

    fn is_valid_key(&self, key: &Key) -> bool;
    fn default_node_and_key(&self) -> Option<(&dyn Node<Key>, &'static Key)>;

    fn default_node(&self) -> Option<&dyn Node<Key>> {
        self.default_node_and_key().map(|(node, _)| node)
    }

    fn default_key(&self) -> Option<&Key> {
        self.default_node_and_key().map(|(_, key)| key)
    }

    fn get_config<'n>(&'n self, key: &Key) -> Option<&'n dyn Any>;
}
