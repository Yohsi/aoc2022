use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Graph<T: Eq + Hash + Copy + Debug> {
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
