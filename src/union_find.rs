#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};
#[derive(Debug)]
struct UnionFind<T> {
    map: HashMap<T, usize>,
    pub array: Vec<usize>
}

impl <T: Eq + Hash> UnionFind<T> {
    pub fn new(nodes: Vec<T>) -> Self {
        let mut map = HashMap::<T, usize>::new();
        let array: Vec<usize> = (0..nodes.len()).collect();

        for (index, node) in nodes.into_iter().enumerate() {
            map.insert(node, index);
        }

        UnionFind { map, array }
    }

    pub fn find(&self, node: usize) -> usize {
        let mut root = self.array[node];

        while self.array[root] != root {
            root = self.array[root]
        }

        return  root;
    }

    pub fn union(&mut self, node1: T, node2: T) {
        if let (Some(&node1_label), Some(&node2_label)) = (self.map.get(&node1), self.map.get(&node2)) {
            let node1_root = self.find(node1_label);
            let node2_root = self.find(node2_label);

            let node1_rank = self.array.iter().filter(|&&x| x == node1_root).count();
            let node2_rank = self.array.iter().filter(|&&x| x == node2_root).count();

            let (parent_node, child_node) = if node1_rank >= node2_rank {
                (node1_root, node2_root)
            } else {
                (node2_root, node1_root)
            };

            self.array[child_node] = parent_node;
        }
    }
}

#[test]
fn union_find() {
    let nodes = Vec::from(["E", "F", "A", "D", "B", "C"]);
    let mut union_find = UnionFind::<&str>::new(nodes);

    union_find.union("A", "C");
    union_find.union("C", "F");
    union_find.union("E", "B");
    union_find.union("B", "D");
    union_find.union("F", "D");

    println!("{:?}", union_find.array)
}