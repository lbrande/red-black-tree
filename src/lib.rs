use core::cell::RefCell;
use core::cmp::Ordering;
use std::rc::Rc;
use std::rc::Weak;

pub struct RBTreeSet<T: Ord> {
    root: Option<WrappedNode<T>>,
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
            self.root = Some(WrappedNode::new(value))
        }
    }
}

struct Node<T: Ord> {
    value: T,
    parent: WeakWrappedNode<T>,
    left_child: Option<WrappedNode<T>>,
    right_child: Option<WrappedNode<T>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T, parent: WeakWrappedNode<T>) -> Self {
        Self {
            value,
            parent,
            left_child: None,
            right_child: None,
        }
    }
}

struct WrappedNode<T: Ord> {
    node: Rc<RefCell<Node<T>>>,
}

impl<T: Ord> WrappedNode<T> {
    fn new(value: T) -> Self {
        Self {
            node: Rc::new(RefCell::new(Node::new(value, WeakWrappedNode::new()))),
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.node.borrow().value) {
            Less => {
                match &mut self.node.borrow_mut().left_child {
                    Some(node) => node.insert(value),
                    None => self.set_left_child(value),
                };
            }
            Equal => return,
            Greater => {
                match &mut self.node.borrow_mut().right_child {
                    Some(node) => node.insert(value),
                    None => self.set_right_child(value),
                };
            }
        }
    }

    fn has_left_child(&self) -> bool {
        self.node.borrow().left_child.is_some()
    }

    fn set_left_child(&self, value: T) {
        self.node.borrow_mut().left_child = Some(Self {
            node: Rc::new(RefCell::new(Node::new(
                value,
                WeakWrappedNode::from_node(self),
            ))),
        });
    }

    fn has_right_child(&self) -> bool {
        self.node.borrow().right_child.is_some()
    }

    fn set_right_child(&self, value: T) {
        self.node.borrow_mut().right_child = Some(Self {
            node: Rc::new(RefCell::new(Node::new(
                value,
                WeakWrappedNode::from_node(self),
            ))),
        });
    }
}

struct WeakWrappedNode<T: Ord> {
    node: Weak<RefCell<Node<T>>>,
}

impl<T: Ord> WeakWrappedNode<T> {
    fn new() -> Self {
        Self { node: Weak::new() }
    }

    fn from_node(parent: &WrappedNode<T>) -> Self {
        Self {
            node: Rc::downgrade(&parent.node),
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
