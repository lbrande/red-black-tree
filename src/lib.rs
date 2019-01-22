use std::cmp::Ordering::*;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
pub struct TreeSet<T: Ord> {
    root: Link<T>,
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
        if self.root.is_some() {
            let node = self.root.as_mut().unwrap();
            let raw = node.as_mut() as *mut Node<T>;
            node.remove(value, raw);
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

    fn remove(&mut self, value: &T, parent: *mut Node<T>) {
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
                            let raw = current.as_mut() as *mut Node<T>;
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
