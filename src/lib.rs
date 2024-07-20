pub mod node;
pub mod node_tree;
pub mod prelude;

// TODO : make bevy stuff a feature for faster testing and compiling

#[cfg(test)]
pub mod tests {
    use std::mem::size_of;

    pub use crate::prelude::*;

    pub struct Player {
        alive: Alive,
        dead: Dead,
    }

    impl Node<str> for Player {
        fn get_node<'n>(&'n self, key: &str) -> Option<&'n dyn Node<str>> {
            Some(match key {
                "alive" => &self.alive,
                "dead" => &self.dead,
                _ => return None,
            })
        }

        fn get_node_mut<'n>(&'n mut self, key: &str) -> Option<&'n mut dyn Node<str>> {
            Some(match key {
                "alive" => &mut self.alive,
                "dead" => &mut self.dead,
                _ => return None,
            })
        }

        fn list_nodes_and_keys<'n>(
            &'n self,
        ) -> Box<dyn Iterator<Item = (&dyn Node<str>, &'static str)> + 'n> {
            let list: [(&dyn Node<_>, _); 2] = [(&self.alive, "alive"), (&self.dead, "dead")];

            Box::new(list.into_iter())
        }

        fn is_valid_key(&self, key: &str) -> bool {
            key == "alive" || key == "dead"
        }

        fn default_node_and_key(&self) -> Option<(&dyn Node<str>, &'static str)> {
            Some((&self.alive, "alive"))
        }
    }

    pub struct Alive {
        grounded: Grounded,
        airborne: Airborne,
    }

    impl Node<str> for Alive {
        fn get_node<'n>(&'n self, key: &str) -> Option<&'n dyn Node<str>> {
            Some(match key {
                "grounded" => &self.grounded,
                "airborne" => &self.airborne,
                _ => return None,
            })
        }

        fn get_node_mut<'n>(&'n mut self, key: &str) -> Option<&'n mut dyn Node<str>> {
            Some(match key {
                "grounded" => &mut self.grounded,
                "airborne" => &mut self.airborne,
                _ => return None,
            })
        }

        fn list_nodes_and_keys<'n>(
            &'n self,
        ) -> Box<dyn Iterator<Item = (&dyn Node<str>, &'static str)> + 'n> {
            let list: [(&dyn Node<_>, _); 2] =
                [(&self.grounded, "grounded"), (&self.airborne, "airborne")];

            Box::new(list.into_iter())
        }

        fn is_valid_key(&self, key: &str) -> bool {
            key == "grounded" || key == "airborne"
        }

        fn default_node_and_key(&self) -> Option<(&dyn Node<str>, &'static str)> {
            Some((&self.grounded, "grounded"))
        }
    }

    pub struct Dead;

    impl Node<str> for Dead {}

    pub struct Grounded {
        movement: Movement,
    }

    impl Node<str> for Grounded {
        fn get_node<'n>(&'n self, key: &str) -> Option<&'n dyn Node<str>> {
            Some(match key {
                "movement" => &self.movement,
                _ => return None,
            })
        }

        fn get_node_mut<'n>(&'n mut self, key: &str) -> Option<&'n mut dyn Node<str>> {
            Some(match key {
                "movement" => &mut self.movement,
                _ => return None,
            })
        }

        fn list_nodes_and_keys<'n>(
            &'n self,
        ) -> Box<dyn Iterator<Item = (&dyn Node<str>, &'static str)> + 'n> {
            let list: [(&dyn Node<_>, _); 1] = [(&self.movement, "movement")];

            Box::new(list.into_iter())
        }

        fn is_valid_key(&self, key: &str) -> bool {
            key == "movement"
        }

        fn default_node_and_key(&self) -> Option<(&dyn Node<str>, &'static str)> {
            Some((&self.movement, "movement"))
        }
    }

    pub struct Airborne {
        jumping: Jumping,
        movement: Movement,
    }

    impl Node<str> for Airborne {
        fn get_node<'n>(&'n self, key: &str) -> Option<&'n dyn Node<str>> {
            Some(match key {
                "jumping" => &self.jumping,
                "movement" => &self.movement,
                _ => return None,
            })
        }

        fn get_node_mut<'n>(&'n mut self, key: &str) -> Option<&'n mut dyn Node<str>> {
            Some(match key {
                "jumping" => &mut self.jumping,
                "movement" => &mut self.movement,
                _ => return None,
            })
        }

        fn list_nodes_and_keys<'n>(
            &'n self,
        ) -> Box<dyn Iterator<Item = (&dyn Node<str>, &'static str)> + 'n> {
            let list: [(&dyn Node<_>, _); 2] =
                [(&self.jumping, "jumping"), (&self.movement, "movement")];

            Box::new(list.into_iter())
        }

        fn is_valid_key(&self, key: &str) -> bool {
            key == "jumping" || key == "movement"
        }

        fn default_node_and_key(&self) -> Option<(&dyn Node<str>, &'static str)> {
            Some((&self.jumping, "jumping"))
        }
    }

    pub struct Jumping {
        movement: Movement,
    }

    impl Node<str> for Jumping {
        fn get_node<'n>(&'n self, key: &str) -> Option<&'n dyn Node<str>> {
            Some(match key {
                "movement" => &self.movement,
                _ => return None,
            })
        }

        fn get_node_mut<'n>(&'n mut self, key: &str) -> Option<&'n mut dyn Node<str>> {
            Some(match key {
                "movement" => &mut self.movement,
                _ => return None,
            })
        }

        fn list_nodes_and_keys<'n>(
            &'n self,
        ) -> Box<dyn Iterator<Item = (&dyn Node<str>, &'static str)> + 'n> {
            let list: [(&dyn Node<_>, _); 1] = [(&self.movement, "movement")];

            Box::new(list.into_iter())
        }

        fn is_valid_key(&self, key: &str) -> bool {
            key == "movement"
        }

        fn default_node_and_key(&self) -> Option<(&dyn Node<str>, &'static str)> {
            Some((&self.movement, "movement"))
        }
    }

    pub struct Movement {
        speed: f32,
    }

    impl Node<str> for Movement {
        fn get_default_config(&self) -> Option<&dyn std::any::Any> {
            Some(&self.speed)
        }
    }

    #[test]
    pub fn test() {
        let player = Player {
            alive: Alive {
                grounded: Grounded {
                    movement: Movement { speed: 10.0 },
                },
                airborne: Airborne {
                    jumping: Jumping {
                        movement: Movement { speed: 8.0 },
                    },
                    movement: Movement { speed: 5.0 },
                },
            },
            dead: Dead,
        };

        assert_eq!(size_of::<Player>(), 12); // I did not expect this was possible!

        // The only memory overhead is the trait pointers to functions, which
        // happen to always be necessary anyway (ignoring inline). The only
        // significant overhead is the downcasting of `any` and corresponding
        // memory which was a core feature I was ready to bear anyway.

        // Worst case you could always cast the node back to its actual type with
        // a single `any` downcast and then run optimal matching on it anyway if
        // you are making like 1000 enemies or something

        // If you really want you can just have the node impl's as optional and
        // treat the `NodeTree` as a typed object to have perfect performance
        // and the potential for more complicated bevaviour later

        let node_tree = NodeTree::new(player);

        let query_shallow_search = node_tree.query().find_node("alive");
        assert!(query_shallow_search.is_some());

        let query_deep_search = node_tree.query().find_node("grounded");
        assert!(query_deep_search.is_some());

        let value_query = *node_tree
            .query()
            .find_node("movement")
            .unwrap()
            .get_default_config_as::<f32>() // it will always return speed regardless of input
            // Typically you would want to actually have a match for readability but it is possible
            .unwrap();

        assert_eq!(value_query, 10.0);

        let failed_query = node_tree.query().find_node("jumping");
        assert!(failed_query.is_none());
    }
}
