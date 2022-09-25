use std::collections::HashSet;

struct ConsistentHashing {
    pub nodes: Vec<Node>,
}

#[derive(Clone)]
struct Node {
    id: i32,
    parent: Option<i32>,
    children: HashSet<i32>,
    keys: HashSet<i32>,
}

impl ConsistentHashing {
    #[allow(non_snake_case)]
    fn new(initialNodes: i32) -> Self {
        let mut node_ids: Vec<i32> = (0..initialNodes).collect();
        node_ids.sort();

        let nodes = node_ids
            .iter()
            .map(|i| Node {
                id: hash(&i),
                parent: None,
                children: HashSet::new(),
                keys: HashSet::new(),
            })
            .collect();

        ConsistentHashing { nodes: nodes }
    }

    fn get_node_for_key(&mut self, key: i32) -> i32 {
        let (target_node_id, mut target_node) = self.get_key_node(key);
        let mut updated_keys = target_node.keys;
        updated_keys.insert(key);
        target_node.keys = updated_keys;
        self.update_node(target_node);
        target_node_id as i32
    }

    fn update_node(&mut self, node: Node) {
        let node_id = node.clone().id;
        for (index, n) in self.nodes.iter().enumerate() {
            if n.id == node_id {
                self.nodes[index] = node;
                break;
            }
        }
    }

    fn get_key_node(&self, key: i32) -> (i32, Node) {
        let mut target_node = None;
        let mut target_node_id = 0;
        let hashed_key = hash(&key);
        for node in &self.nodes {
            if target_node_id <= hashed_key && node.id > hashed_key {
                break;
            } else {
                target_node_id = node.id;
                target_node = Some(node.clone());
            }
        }
        (hash(&target_node_id), target_node.unwrap())
    }

    fn remove_node(&mut self, node: i32) -> i32 {
        let node_to_be_removed = hash(&node);
        let mut last_node = 0;
        for (index, node) in self.nodes.iter().enumerate() {
            if node_to_be_removed == node.id {
                self.nodes.remove(index);
                break;
            } else {
                last_node += node.id;
            }
        }
        hash(&last_node)
    }

    fn add_node(&mut self) -> Vec<i32> {
        // Get required information
        let new_node_id: i32 = self.nodes.len() as i32;
        let mut parent_node = self.get_node_for_replica();

        // Update parent
        let parent_id = &parent_node.id;
        parent_node.children.insert(new_node_id);
        self.update_node(parent_node.clone());

        // Add new node to cluster
        let new_node = Node {
            id: new_node_id,
            parent: Some(parent_id.clone()),
            children: HashSet::new(),
            keys: parent_node.keys,
        };
        self.nodes.push(new_node);

        vec![new_node_id, parent_id.clone()]
    }

    fn get_keys_in_node(&self, node: i32) -> Vec<i32> {
        let mut keys = Vec::new();
        for n in &self.nodes {
            if n.id == node {
                let mut k: Vec<i32> = n.keys.clone().into_iter().collect();
                keys.append(&mut k)
            }
        }
        keys
    }

    fn get_node_for_replica(&self) -> Node {
        let mut ordered_nodes = self.nodes.clone();
        ordered_nodes.sort_by(|a, b| a.children.len().cmp(&b.children.len()));
        ordered_nodes.first().unwrap().to_owned()
    }

    fn get_last_primary_node(&self) -> Node {
        let mut last: Option<Node> = None;
        for node in &self.nodes {
            match &last {
                Some(n) => {
                    if n.parent.is_none() && n.id > node.id {
                        last = Some(node.clone())
                    }
                }
                None => last = Some(node.clone()),
            }
        }
        last.unwrap()
    }
}

fn hash(key: &i32) -> i32 {
    key.swap_bytes()
}

#[cfg(test)]
mod consistent_hashing_tests {
    use super::*;

    #[test]
    fn test_inserts_nodes() {
        let ch = ConsistentHashing::new(5);
        assert_eq!(ch.nodes.len(), 5)
    }

    #[test]
    fn test_get_node_inserts() {
        let mut ch = ConsistentHashing::new(1);
        ch.get_node_for_key(10);
        match ch.nodes.first() {
            Some(node) => {
                assert!(node.keys.contains(&10))
            }
            None => panic!(),
        }
    }

    #[test]
    fn test_same_key_same_node() {
        let mut ch = ConsistentHashing::new(6);
        let first_result = ch.get_node_for_key(10);
        let second_result = ch.get_node_for_key(10);
        assert_eq!(first_result, second_result)
    }
    #[test]
    fn test_get_node_above_range() {
        let mut ch = ConsistentHashing::new(5);
        let node = ch.get_node_for_key(13);
        let _original_node_number = hash(&node);
        assert_eq!(node, 4)
    }
    #[test]
    fn test_hash_reverses() {
        let initial_value = 3;
        let hashed = hash(&initial_value);
        let re_reversed = hash(&hashed);
        assert_eq!(initial_value, re_reversed)
    }
}
