use core::cell::Ref;
use core::cell::RefCell;
use core::cell::RefMut;
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

    fn insert(&self, value: T) {
        let mut raw = self.as_mut();
        match value.cmp(&raw.value) {
            Less => {
                if let Some(node) = &raw.left_child {
                    node.insert(value);
                } else {
                    raw.left_child = Some(Self {
                        raw: Rc::new(RefCell::new(RawNode::new(value, WeakNode::from_node(self)))),
                    });
                }
            }
            Equal => return,
            Greater => {
                if let Some(node) = &raw.right_child {
                    node.insert(value);
                } else {
                    raw.right_child = Some(Self {
                        raw: Rc::new(RefCell::new(RawNode::new(value, WeakNode::from_node(self)))),
                    });
                }
            }
        }
    }

    fn remove(&self, value: &T) {
        let ordering = value.cmp(&self.as_ref().value);
        match ordering {
            Less => {
                if let Some(node) = &self.as_ref().left_child {
                    node.remove(value);
                }
            }
            Equal => {
                if self.as_ref().left_child.is_some() {
                    /*let max = self.as_ref().left_child.unwrap().max();
                    if let Some(left) = &max.as_ref().left_child {
                        left.as_mut().parent = max.as_ref().parent.clone();
                        max.as_ref().parent.to_node().unwrap().as_mut().right_child =
                            max.as_mut().left_child.take();
                    }
                    if let Some(right) = &self.as_ref().right_child {
                        right.as_mut().parent = WeakNode::from_node(&max);
                    }
                    max.as_mut().right_child = self.as_mut().right_child.take();
                    if let Some(left) = &self.as_ref().left_child {
                        left.as_mut().parent = WeakNode::from_node(&max);
                    }
                    max.as_mut().left_child = self.as_mut().left_child.take();
                    max.as_mut().parent = self.as_ref().parent.clone();
                    if *value < self.as_ref().parent.to_node().unwrap().as_ref().value {
                        self.as_ref().parent.to_node().unwrap().as_mut().left_child = Some(max);
                    } else {
                        self.as_ref().parent.to_node().unwrap().as_mut().right_child = Some(max);
                    }*/
                } else {
                    /*if let Some(node) = &self.as_ref().right_child {
                        node.as_mut().parent = self.as_ref().parent.clone();
                    }*/
                    let less = *value < self.as_ref().parent.to_node().unwrap().as_ref().value;
                    if less {
                        /*self.as_ref().parent.to_node().unwrap().as_mut().left_child =
                            self.as_mut().right_child.take();*/
                    } else {
                        self.as_ref().parent.to_node().unwrap().as_mut().right_child =
                            self.as_mut().right_child.take();
                    }
                }
            }
            Greater => {
                if let Some(node) = &self.as_ref().right_child {
                    node.remove(value);
                }
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        let raw = self.as_ref();
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

    fn as_ref(&self) -> Ref<RawNode<T>> {
        self.raw.borrow()
    }

    fn as_mut(&self) -> RefMut<RawNode<T>> {
        self.raw.borrow_mut()
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

    fn to_node(&self) -> Option<Node<T>> {
        self.raw.upgrade().map(|r| Node { raw: r })
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

        set.remove(&15);

        assert_eq!(set.contains(&15), false);
    }
}
