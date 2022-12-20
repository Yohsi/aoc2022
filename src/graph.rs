use std::cmp::{min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::Add;

pub trait Graph<T: Eq + Hash + Copy> {
    fn neighbors(&self, node: T) -> Vec<T>;

    fn route(&self, from: T, to: T) -> Option<VecDeque<T>> {
        let mut visited = HashMap::new(); // Node, parent
        let mut todo = VecDeque::new();
        todo.push_back(from);
        visited.insert(from, None);
        let mut found = false;

        while !todo.is_empty() && !found {
            let current = todo.pop_front().unwrap();
            if current == to {
                break;
            }
            for neighbor in self.neighbors(current) {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, Some(current));
                    if neighbor == to {
                        found = true;
                        break;
                    }
                    todo.push_back(neighbor);
                }
            }
        }
        if !found {
            return None;
        }

        let mut current = to;
        let mut path = VecDeque::new();
        while current != from {
            path.push_front(current);
            current = visited[&current].unwrap();
        }
        Some(path)
    }
}

#[derive(Eq, PartialEq)]
struct NodeCost<T: Eq, C: Ord> {
    node: T,
    cost: C,
}

impl<T: Eq, C: Ord> Ord for NodeCost<T, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl<T: Eq, C: Ord> PartialOrd for NodeCost<T, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

pub trait CostGraph<T: Eq + Hash + Copy, C: Ord + Add<Output=C> + Copy + From<u8>>: Graph<T> {
    fn cost(&self, from: T, to: T) -> C;

    fn dijkstra(&self, from: T) -> HashMap<T, C> {
        let mut costs = HashMap::new();
        let mut todo = BinaryHeap::new();
        let mut visited = HashSet::new();

        todo.push(Reverse(NodeCost { node: from, cost: 0.into() }));

        while let Some(node_cost) = todo.pop() {
            let cost: C = node_cost.0.cost;
            let node = node_cost.0.node;
            visited.insert(node);
            for &neighbor in self.neighbors(node).iter()
                .filter(|n| !visited.contains(n)) {
                let calculated_cost = cost + self.cost(node, neighbor);
                costs.entry(neighbor).and_modify(|c| *c = min(*c, calculated_cost)).or_insert(calculated_cost);
                todo.push(Reverse(NodeCost { node: neighbor, cost: calculated_cost }));
            }
        }

        costs
    }
}