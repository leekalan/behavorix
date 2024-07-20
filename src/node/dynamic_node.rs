use crate::prelude::*;

pub(crate) struct DynNode<'n, Key: ?Sized + PartialEq + Eq + 'static>(pub &'n dyn Node<Key>);

impl<'n, Key: ?Sized + PartialEq + Eq + 'static> DynNode<'n, Key> {
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
