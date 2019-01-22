use std::cmp::Ordering::*;
use std::mem;
use std::ptr::*;

pub struct IntoIter<T: Ord> {
    next: Link<T>,
    next_parent: *mut Node<T>,
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(node) = self.next.take() {
            let next = node.next(self.next_parent);
            self.next_parent = next.1;
            self.next = next.0;
            self.next.take().map(|node| node.value)
        } else {
            None
        }
    }
}

impl<T: Ord> IntoIter<T> {
    fn new(node: Link<T>) -> Self {
        if let Some(node) = node {
            let mut current = node;
            let mut current_parent = null_mut();
            loop {
                let raw = current.as_mut() as *mut Node<T>;
                if let Some(node) = current.left_child {
                    current = node;
                    current_parent = raw;
                } else {
                    break;
                }
            }
            Self {
                next: Some(current),
                next_parent: current_parent,
            }
        } else {
            Self {
                next: None,
                next_parent: null_mut(),
            }
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
pub struct TreeSet<T: Ord> {
    root: Link<T>,
}

impl<T: Ord> IntoIterator for TreeSet<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self.root)
    }
}

impl<T: Ord> TreeSet<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        if let Some(node) = &mut self.root {
            node.insert(value);
        } else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }

    pub fn remove(&mut self, value: &T) {
        if let Some(node) = &mut self.root {
            if *value == node.value {
                self.root = node.remove_as_root();
            } else {
                node.remove(value, null_mut());
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        if let Some(node) = &self.root {
            node.contains(value)
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left_child: Link<T>,
    right_child: Link<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left_child: None,
            right_child: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Less => {
                if let Some(node) = &mut self.left_child {
                    node.insert(value);
                } else {
                    self.left_child = Some(Box::new(Node::new(value)));
                }
            }
            Equal => return,
            Greater => {
                if let Some(node) = &mut self.right_child {
                    node.insert(value);
                } else {
                    self.right_child = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    fn remove(&mut self, value: &T, parent: *mut Self) {
        match value.cmp(&self.value) {
            Less => {
                let raw = self as *mut Self;
                if let Some(node) = &mut self.left_child {
                    node.remove(value, raw);
                }
            }
            Equal => {
                if let Some(parent) = unsafe { parent.as_mut() } {
                    let raw = self as *mut Self;
                    if let Some(node) = &mut self.left_child {
                        let mut current = node;
                        let mut current_parent = raw;
                        loop {
                            let raw = current.as_mut() as *mut Self;
                            if let Some(node) = &mut current.right_child {
                                current = node;
                                current_parent = raw;
                            } else {
                                break;
                            }
                        }
                        if let Some(current_parent) = unsafe { current_parent.as_mut() } {
                            if let Some(mut current) = current_parent.right_child.take() {
                                current_parent.right_child = current.left_child;
                                current.left_child = self.left_child.take();
                                current.right_child = self.right_child.take();
                                if *value < parent.value {
                                    parent.left_child = Some(current);
                                } else {
                                    parent.right_child = Some(current);
                                }
                            }
                        }
                    } else if *value < parent.value {
                        parent.left_child = self.right_child.take();
                    } else {
                        parent.right_child = self.right_child.take();
                    }
                }
            }
            Greater => {
                let raw = self as *mut Self;
                if let Some(node) = &mut self.right_child {
                    node.remove(value, raw);
                }
            }
        }
    }

    fn remove_as_root(&mut self) -> Link<T> {
        let raw = self as *mut Self;
        if let Some(node) = &mut self.left_child {
            let mut current = node;
            let mut current_parent = raw;
            loop {
                let raw = current.as_mut() as *mut Self;
                if let Some(node) = &mut current.right_child {
                    current = node;
                    current_parent = raw;
                } else {
                    break;
                }
            }
            if let Some(current_parent) = unsafe { current_parent.as_mut() } {
                if let Some(mut current) = current_parent.right_child.take() {
                    current_parent.right_child = current.left_child;
                    current.left_child = self.left_child.take();
                    current.right_child = self.right_child.take();
                    return Some(current);
                }
            }
        }
        self.right_child.take()
    }

    fn contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Less => {
                if let Some(node) = &self.left_child {
                    node.contains(value)
                } else {
                    false
                }
            }
            Equal => true,
            Greater => {
                if let Some(node) = &self.right_child {
                    node.contains(value)
                } else {
                    false
                }
            }
        }
    }

    fn next(self, parent: *mut Self) -> (Link<T>, *mut Self) {
        (None, null_mut())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tree_set_test() {
        let mut set = TreeSet::new();

        set.insert(6);
        set.insert(3);
        set.insert(0);
        set.insert(7);
        set.insert(4);
        set.insert(1);
        set.insert(8);
        set.insert(5);
        set.insert(2);
        set.insert(9);

        assert!(set.contains(&6));
        set.remove(&6);
        assert!(!set.contains(&6));
        assert!(set.contains(&3));
        set.remove(&3);
        assert!(!set.contains(&3));
        assert!(set.contains(&0));
        set.remove(&0);
        assert!(!set.contains(&0));
        assert!(set.contains(&7));
        set.remove(&7);
        assert!(!set.contains(&7));
        assert!(set.contains(&4));
        set.remove(&4);
        assert!(!set.contains(&4));
        assert!(set.contains(&1));
        set.remove(&1);
        assert!(!set.contains(&1));
        assert!(set.contains(&8));
        set.remove(&8);
        assert!(!set.contains(&8));
        assert!(set.contains(&5));
        set.remove(&5);
        assert!(!set.contains(&5));
        assert!(set.contains(&2));
        set.remove(&2);
        assert!(!set.contains(&2));
        assert!(set.contains(&9));
        set.remove(&9);
        assert!(!set.contains(&9));
    }
}
