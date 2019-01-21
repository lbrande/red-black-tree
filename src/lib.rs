use core::cell::RefCell;
use core::cmp::Ordering::*;
use std::rc::Rc;
use std::rc::Weak;

pub struct RBTreeSet<T: Ord> {
    root: Option<Node<T>>,
}

impl<T: Ord> Default for RBTreeSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> RBTreeSet<T> {
    pub fn new() -> Self {
        RBTreeSet { root: None }
    }

    pub fn insert(&mut self, value: T) {
        if let Some(node) = &mut self.root {
            node.insert(value);
        } else {
            self.root = Some(Node::new(value));
        }
    }

    pub fn remove(&mut self, value: &T) {
        if let Some(node) = &mut self.root {
            node.remove(value);
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

struct Node<T: Ord> {
    raw: Rc<RefCell<RawNode<T>>>,
}

impl<T: Ord> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            raw: Rc::clone(&self.raw),
        }
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            raw: Rc::new(RefCell::new(RawNode::new(value, WeakNode::new()))),
        }
    }

    fn insert(&mut self, value: T) {
        let mut raw = self.raw.borrow_mut();
        match value.cmp(&raw.value) {
            Less => {
                if let Some(node) = &mut raw.left_child {
                    node.insert(value);
                } else {
                    raw.left_child = Some(Self {
                        raw: Rc::new(RefCell::new(RawNode::new(value, WeakNode::from_node(self)))),
                    });
                }
            }
            Equal => return,
            Greater => {
                if let Some(node) = &mut raw.right_child {
                    node.insert(value);
                } else {
                    raw.right_child = Some(Self {
                        raw: Rc::new(RefCell::new(RawNode::new(value, WeakNode::from_node(self)))),
                    });
                }
            }
        }
    }

    fn remove(&mut self, value: &T) {
        let mut raw = self.raw.borrow_mut();
        match value.cmp(&raw.value) {
            Less => {
                if let Some(node) = &mut raw.left_child {
                    node.remove(value);
                }
            }
            Equal => {
                let parent = raw.parent.raw.upgrade().unwrap();
                let mut parent_raw = parent.borrow_mut();
                if let Some(node) = &raw.left_child {
                    let node = node.max();
                    if let Some(right) = &raw.right_child {
                        right.raw.borrow_mut().parent = WeakNode::from_node(&node);
                    }
                    node.raw.borrow_mut().right_child = raw.right_child.take();
                    if let Some(left) = &node.raw.borrow().left_child {
                        left.raw.borrow_mut().parent = node.raw.borrow().parent.clone();
                        node.raw
                            .borrow()
                            .parent
                            .raw
                            .upgrade()
                            .unwrap()
                            .borrow_mut()
                            .right_child = node.raw.borrow_mut().left_child.take();
                    }
                    if let Some(left) = &raw.left_child {
                        left.raw.borrow_mut().parent = WeakNode::from_node(&node);
                    }
                    node.raw.borrow_mut().left_child = raw.left_child.take();
                    node.raw.borrow_mut().parent = raw.parent.clone();
                    if *value < parent_raw.value {
                        parent_raw.left_child = Some(node);
                    } else {
                        parent_raw.right_child = Some(node);
                    }
                } else {
                    if let Some(node) = &raw.right_child {
                        node.raw.borrow_mut().parent = raw.parent.clone();
                    }
                    if *value < parent_raw.value {
                        parent_raw.left_child = raw.right_child.take();
                    } else {
                        parent_raw.right_child = raw.right_child.take();
                    }
                }
            }
            Greater => {
                if let Some(node) = &mut raw.right_child {
                    node.remove(value);
                }
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        let raw = self.raw.borrow();
        match value.cmp(&raw.value) {
            Less => {
                if let Some(node) = &raw.left_child {
                    node.contains(value)
                } else {
                    false
                }
            }
            Equal => true,
            Greater => {
                if let Some(node) = &raw.right_child {
                    node.contains(value)
                } else {
                    false
                }
            }
        }
    }

    fn max(&self) -> Node<T> {
        if let Some(node) = &self.raw.borrow().right_child {
            node.max()
        } else {
            self.clone()
        }
    }
}

struct WeakNode<T: Ord> {
    raw: Weak<RefCell<RawNode<T>>>,
}

impl<T: Ord> Clone for WeakNode<T> {
    fn clone(&self) -> Self {
        Self {
            raw: Weak::clone(&self.raw),
        }
    }
}

impl<T: Ord> WeakNode<T> {
    fn new() -> Self {
        Self { raw: Weak::new() }
    }

    fn from_node(parent: &Node<T>) -> Self {
        Self {
            raw: Rc::downgrade(&parent.raw),
        }
    }
}

struct RawNode<T: Ord> {
    value: T,
    parent: WeakNode<T>,
    left_child: Option<Node<T>>,
    right_child: Option<Node<T>>,
}

impl<T: Ord> RawNode<T> {
    fn new(value: T, parent: WeakNode<T>) -> Self {
        Self {
            value,
            parent,
            left_child: None,
            right_child: None,
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

        set.remove(&10);
    }
}
