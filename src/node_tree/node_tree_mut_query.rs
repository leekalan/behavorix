use crate::prelude::*;

use std::any::Any;

pub struct NodeTreeMutQuery<'n, 'key, Key: ?Sized + PartialEq + Eq + 'static> {
    pub(super) node: &'n dyn Node<Key>,
    pub(super) keys: KeyIteratorMut<'n, 'key, Key>,
}

impl<'n, 'key, Key: ?Sized + PartialEq + Eq + 'static> NodeTreeMutQuery<'n, 'key, Key> {
    pub fn node(&'n self) -> &'n dyn Node<Key> {
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

    pub fn find_node(self, comp_key: &Key) -> Option<Self> {
        let mut keys = self.keys;

        let mut node = self.node;

        while let Some(key) = keys.next() {
            if **key != *comp_key {
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

    #[allow(clippy::while_let_on_iterator)]
    pub fn change_nodes(&mut self, from: &[&'key Key], to: &[&'key Key]) -> Option<()> {
        // GETTING THE POSITION TO BE COPYING FROM

        let mut path = self.keys.as_key_iterator();

        'from: for key in from {
            while let Some(p) = path.next() {
                if **key == *p {
                    continue 'from;
                }
            }

            return None;
        }

        let path_truncate = path.index;

        // TODO MOVING TO THE PLACE TO BE PASTING

        // The hard part is I need to traverse around the query trees to find each of
        // the `to` nodes. Sadly this seems like it will be quite inefficient.
        //
        // There will be quite alot of reliance upon the user being direct with the
        // `to` nodes for the more spread out nodes, such as rather than setting `to`
        // as &["jumping"] it would be way more efficient to do &["airborne", "jumping"].
        // This aproach will be linear time aslong as there is high specification.
        // Otherwise it will probably be like exponential on N where N is the gap between
        // specifications

        let mut node = self.as_node_tree_query();

        // for key in to {
        //     let Valid::Valid(n) = node.find_node(key) else {
        //         return None;
        //     };

        //     node = n;
        // }

        // ADDING TO THE PASTE LOCATION

        let mut new_path = Vec::new();

        let mut node = node.node;

        'to: for key in path {
            if let Some(n) = node.get_node(key) {
                new_path.push(key);
                node = n;
                continue 'to;
            } else {
                break 'to;
            }
        }

        new_path.append(&mut DynNode(node).default_keys());

        // TRUNACTE AND APPEND NEW STUFF

        self.keys.keys.truncate(path_truncate);

        self.keys.keys.append(&mut new_path);

        Some(())
    }
}
