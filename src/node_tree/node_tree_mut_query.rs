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

        let path_truncate = path.index + 1;

        'from: for key in from {
            while let Some(p) = path.next() {
                if **key == *p {
                    continue 'from;
                }
            }

            return None;
        }

        // MOVING TO THE PLACE TO BE PASTING

        let mut new_path = Vec::new();

        // As I did not want to implement tree traversal as there are 2 potential options
        // 1. A very space heavy full of allocation BFS
        // 2. A very time consuming node traversal DFS

        // Therefore I decided that I will just use the default node and if the user
        // wants to change to something that is not default, they need to specify the
        // path

        let mut node = self.node;

        'f: for key in to {
            'l: loop {
                if let Some(n) = node.get_node(key) {
                    new_path.push(*key);
                    node = n;
                    continue 'f;
                } else {
                    let (default_node, default_key) = node.default_node_and_key()?;

                    new_path.push(default_key);
                    node = default_node;
                    continue 'l;
                }
            }
        }

        // ADDING TO THE PASTE LOCATION

        'paste: for key in path {
            // Remainder of path that will be discarded
            // This is to try maintain the same path from the new node as it only has linear time
            // Just using the default nodes takes the same time complexity
            if let Some(n) = node.get_node(key) {
                new_path.push(key);
                node = n;
                continue 'paste;
            } else {
                break 'paste;
            }
        }

        new_path.append(&mut DynNode(node).default_keys());

        // TRUNACTE AND APPEND NEW STUFF

        self.keys.keys.truncate(path_truncate);

        self.keys.keys.append(&mut new_path);

        Some(())
    }
}
