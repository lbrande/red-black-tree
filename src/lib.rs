use std::cmp::Ordering::*;

pub struct RBTreeSet<T: Ord> {
    nodes: Vec<Option<T>>,
}

impl<T: Ord> Default for RBTreeSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> RBTreeSet<T> {
    pub fn new() -> Self {
        RBTreeSet { nodes: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        let mut index = 0;

        while let Some(other) = self.get(index) {
            match value.cmp(other) {
                Less => index = self.left(index),
                Equal => return,
                Greater => index = self.right(index),
            }
        }

        self.set(index, Some(value));
    }

    pub fn remove(&mut self, value: T) {
        let mut index = 0;

        while let Some(other) = self.get(index) {
            match value.cmp(other) {
                Less => index = self.left(index),
                Equal => {
                    return;
                }
                Greater => index = self.right(index),
            }
        }

        self.extend_to(index + 1);

        self.nodes[index] = Some(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut index = 0;

        while let Some(other) = self.get(index) {
            match value.cmp(other) {
                Less => index = self.left(index),
                Equal => return true,
                Greater => index = self.right(index),
            }
        }

        false
    }

    fn left(&self, index: usize) -> usize {
        index * 2 + 1
    }

    fn right(&self, index: usize) -> usize {
        index * 2 + 2
    }

    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    fn grandparent(&self, index: usize) -> usize {
        (index - 3) / 4
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.nodes.len() {
            self.nodes[index].as_ref()
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, value: Option<T>) {
        self.extend_to(index + 1);
        self.nodes[index] = value
    }

    fn take(&mut self, index: usize) -> Option<T> {
        if index < self.nodes.len() {
            self.nodes[index].take()
        } else {
            None
        }
    }

    fn extend_to(&mut self, size: usize) {
        while self.nodes.len() < size {
            self.nodes.push(None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_test() {
        let mut set = RBTreeSet::new();
        set.insert(10);
        set.insert(15);
        set.insert(4);
        set.insert(7);

        assert_eq!(set.contains(&7), true);
        assert_eq!(set.contains(&10), true);
        assert_eq!(set.contains(&15), true);
        assert_eq!(set.contains(&3), false);
    }
}
