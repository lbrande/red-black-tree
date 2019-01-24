use std::cmp::Ordering::*;
use std::ptr::*;

type Link<T> = Option<Box<Node<T>>>;
type UnsafeLink<T> = *mut Node<T>;

#[derive(Default)]
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
            self.root.as_mut().unwrap().color = true;
        }
    }

    pub fn remove(&mut self, value: &T) {
        if let Some(node) = &mut self.root {
            if node.left_child.is_none() && node.right_child.is_none() {
                self.root = None;
            } else {
                node.remove(value);
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

    pub fn min(&self) -> Option<&T> {
        if let Some(node) = &self.root {
            Some(&node.min().value)
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<&T> {
        if let Some(node) = &self.root {
            Some(&node.max().value)
        } else {
            None
        }
    }
}

struct Node<T: Ord> {
    value: T,
    color: bool,
    left_child: Link<T>,
    right_child: Link<T>,
    parent: UnsafeLink<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            color: false,
            left_child: None,
            right_child: None,
            parent: null_mut(),
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Less => {
                if let Some(node) = &mut self.left_child {
                    node.insert(value);
                } else {
                    self.set_left_child(Some(Box::new(Node::new(value))));
                    self.left_child.as_mut().unwrap().balance_after_insert();
                }
            }
            Equal => return,
            Greater => {
                if let Some(node) = &mut self.right_child {
                    node.insert(value);
                } else {
                    self.set_right_child(Some(Box::new(Node::new(value))));
                    self.right_child.as_mut().unwrap().balance_after_insert();
                }
            }
        }
    }

    fn remove(&mut self, value: &T) {
        match value.cmp(&self.value) {
            Less => {
                if let Some(node) = &mut self.left_child {
                    node.remove(value);
                }
            }
            Equal => {
                if self.parent.is_null() {
                    self.remove_without_parent();
                } else {
                    self.remove_with_parent();
                }
            }
            Greater => {
                if let Some(node) = &mut self.right_child {
                    node.remove(value);
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

    fn min(&self) -> &Self {
        if let Some(node) = &self.left_child {
            node.min()
        } else {
            self
        }
    }

    fn max(&self) -> &Self {
        if let Some(node) = &self.right_child {
            node.max()
        } else {
            self
        }
    }

    fn balance_after_insert(&mut self) {

    }

    fn remove_with_parent(&mut self) {
        if let Some(node) = &mut self.left_child {
            if let Some(current_parent) = node.max().parent() {
                if let Some(mut current) = current_parent.right_child.take() {
                    current_parent.set_right_child(current.left_child.take());
                    current.set_left_child(self.left_child.take());
                    current.set_right_child(self.right_child.take());
                    self.replace(Some(current));
                }
            }
        } else {
            let right_child = self.right_child.take();
            self.replace(right_child);
        }
    }

    fn remove_without_parent(&mut self) {
        if let Some(node) = &mut self.left_child {
            if let Some(current_parent) = node.max().parent() {
                if let Some(current) = current_parent.right_child.take() {
                    current_parent.set_right_child(current.left_child);
                    self.value = current.value;
                }
            }
        } else if let Some(node) = self.right_child.take() {
            self.value = node.value;
            self.set_left_child(node.left_child);
            self.set_right_child(node.right_child);
        }
    }

    fn replace(&self, child: Link<T>) {
        if !self.parent.is_null() {
            unsafe {
                if self.value < (*self.parent).value {
                    (*self.parent).set_left_child(child);
                } else {
                    (*self.parent).set_right_child(child);
                }
            }
        }
    }

    fn parent(&self) -> Option<&mut Self> {
        unsafe { self.parent.as_mut() }
    }

    fn grandparent(&self) -> Option<&mut Self> {
        self.parent().and_then(|node| node.parent())
    }

    fn sibling(&self) -> Option<&mut Self> {
        if let Some(parent) = self.parent() {
            (if self.value < parent.value {
                &mut parent.right_child
            } else {
                &mut parent.left_child
            })
            .as_mut()
            .map(|node| node.as_mut())
        } else {
            None
        }
    }

    fn uncle(&self) -> Option<&mut Self> {
        self.parent().and_then(|node| node.sibling())
    }

    fn set_left_child(&mut self, mut node: Link<T>) {
        if let Some(node) = &mut node {
            node.parent = self as UnsafeLink<T>;
        }
        self.left_child = node;
    }

    fn set_right_child(&mut self, mut node: Link<T>) {
        if let Some(node) = &mut node {
            node.parent = self as UnsafeLink<T>;
        }
        self.right_child = node;
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
