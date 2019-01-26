use crate::LinkColor::*;
use std::cmp::Ordering::*;
use std::mem;
use std::ptr::*;

type Link<T> = Option<Box<Node<T>>>;
type UnsafeLink<T> = *mut Node<T>;

#[derive(Copy, Clone, Debug, PartialEq)]
enum LinkColor {
    Red,
    Black,
}

#[derive(Debug, Default)]
pub struct TreeSet<T: Ord> {
    root: Link<T>,
}

impl<T: Ord> TreeSet<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let root = &mut self.root as *mut Link<T>;
        if let Some(node) = &mut self.root {
            node.insert(value, root);
        } else {
            self.root = Some(Box::new(Node::new(value, root)));
            self.root.as_mut().unwrap().color = Black;
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

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    color: LinkColor,
    left_child: Link<T>,
    right_child: Link<T>,
    parent: UnsafeLink<T>,
    root: *mut Link<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T, root: *mut Link<T>) -> Self {
        Self {
            value,
            color: Red,
            left_child: None,
            right_child: None,
            parent: null_mut(),
            root,
        }
    }

    fn insert(&mut self, value: T, root: *mut Link<T>) {
        match value.cmp(&self.value) {
            Less => {
                if let Some(node) = &mut self.left_child {
                    node.insert(value, root);
                } else {
                    self.set_left_child(Some(Box::new(Node::new(value, root))));
                    Node::balance_after_insert(self.left_child.as_mut().map(|node| node.as_mut()));
                }
            }
            Equal => return,
            Greater => {
                if let Some(node) = &mut self.right_child {
                    node.insert(value, root);
                } else {
                    self.set_right_child(Some(Box::new(Node::new(value, root))));
                    Node::balance_after_insert(self.right_child.as_mut().map(|node| node.as_mut()));
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
            Equal => self.remove_self(),
            Greater => {
                if let Some(node) = &mut self.right_child {
                    node.remove(value);
                }
            }
        }
    }

    fn remove_self(&mut self) {
        if self.left_child.is_some() && self.right_child.is_some() {
            let succ = self.right_child.as_ref().unwrap().min().as_mut();
            mem::swap(&mut self.value, &mut succ.value);
            succ.remove_self();
        } else if let Some(node) = self.left_child.take() {
            self.replace(Some(node));
        } else if let Some(node) = self.right_child.take() {
            self.replace(Some(node));
        } else {
            self.replace(None);
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

    fn parent(&self) -> Option<&mut Self> {
        unsafe { self.parent.as_mut() }
    }

    fn grandparent(&self) -> Option<&mut Self> {
        self.parent().and_then(|node| node.parent())
    }

    fn sibling(&self) -> Option<&mut Self> {
        if let Some(parent) = self.parent() {
            (if parent.left_child.is_some()
                && parent.left_child.as_ref().unwrap().as_ref() as *const Self
                    == self as *const Self
            {
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

    fn replace(&self, mut child: Link<T>) {
        if self.parent.is_null() {
            unsafe {
                if let Some(node) = &mut child {
                    node.parent = null_mut();
                }
                *self.root = child;
            }
        } else {
            unsafe {
                if (*self.parent).left_child.is_some()
                    && (*self.parent).left_child.as_ref().unwrap().as_ref() as *const Self
                        == self as *const Self
                {
                    (*self.parent).set_left_child(child);
                } else {
                    (*self.parent).set_right_child(child);
                }
            }
        }
    }

    fn take(&self) -> Box<Self> {
        if self.parent.is_null() {
            unsafe { (*self.root).take().unwrap() }
        } else {
            unsafe {
                if (*self.parent).left_child.is_some()
                    && (*self.parent).left_child.as_ref().unwrap().as_ref() as *const Self
                        == self as *const Self
                {
                    (*self.parent).left_child.take().unwrap()
                } else {
                    (*self.parent).right_child.take().unwrap()
                }
            }
        }
    }

    fn as_mut(&self) -> &mut Box<Self> {
        if self.parent.is_null() {
            unsafe { (*self.root).as_mut().unwrap() }
        } else {
            unsafe {
                if (*self.parent).left_child.is_some()
                    && (*self.parent).left_child.as_ref().unwrap().as_ref() as *const Self
                        == self as *const Self
                {
                    (*self.parent).left_child.as_mut().unwrap()
                } else {
                    (*self.parent).right_child.as_mut().unwrap()
                }
            }
        }
    }

    fn make_root(mut node: Box<Self>) {
        unsafe {
            let root = node.root;
            node.parent = null_mut();
            *root = Some(node);
        }
    }

    fn make_child_of(node: Box<Self>, parent: UnsafeLink<T>) {
        unsafe {
            if node.value < (*parent).value {
                (*parent).set_left_child(Some(node));
            } else {
                (*parent).set_right_child(Some(node));
            }
        }
    }

    fn balance_after_insert(node: Option<&mut Self>) {
        if let Some(node) = node {
            if node.parent().is_none() {
                node.color = Black;
            }
            if Node::get_color(node.parent()) == Red {
                match Node::get_color(node.uncle()) {
                    Red => {
                        Node::set_color(node.parent(), Black);
                        Node::set_color(node.uncle(), Black);
                        Node::set_color(node.grandparent(), Red);
                        Node::balance_after_insert(node.grandparent());
                    }
                    Black => {
                        if Node::is_left_child(Some(node)) {
                            if Node::is_left_child(node.parent()) {
                                Node::set_color(node.parent(), Black);
                                Node::set_color(node.grandparent(), Red);
                                Node::rotate_right(node.grandparent());
                            } else {
                                Node::rotate_right(node.parent());
                                Node::balance_after_insert(
                                    node.right_child.as_mut().map(|node| node.as_mut()),
                                );
                            }
                        } else if Node::is_left_child(node.parent()) {
                            Node::rotate_left(node.parent());
                            Node::balance_after_insert(
                                node.left_child.as_mut().map(|node| node.as_mut()),
                            );
                        } else {
                            Node::set_color(node.parent(), Black);
                            Node::set_color(node.grandparent(), Red);
                            Node::rotate_left(node.grandparent());
                        }
                    }
                }
            }
        }
    }

    fn get_color(node: Option<&mut Self>) -> LinkColor {
        if let Some(node) = node {
            node.color
        } else {
            Black
        }
    }

    fn set_color(node: Option<&mut Self>, color: LinkColor) {
        if let Some(node) = node {
            node.color = color;
        }
    }

    fn is_left_child(node: Option<&mut Self>) -> bool {
        if let Some(node) = node {
            if let Some(parent) = node.parent() {
                return node.value < parent.value;
            }
        }
        false
    }

    fn rotate_left(node: Option<&mut Self>) {
        if let Some(node) = node {
            if let Some(mut child) = node.right_child.take() {
                node.set_right_child(child.left_child.take());
                if node.parent().is_some() {
                    let parent = node.parent;
                    child.set_left_child(Some(node.take()));
                    Node::make_child_of(child, parent);
                } else {
                    child.set_left_child(Some(node.take()));
                    Node::make_root(child);
                }
            }
        }
    }

    fn rotate_right(node: Option<&mut Self>) {
        if let Some(node) = node {
            if let Some(mut child) = node.left_child.take() {
                node.set_left_child(child.right_child.take());
                if node.parent().is_some() {
                    let parent = node.parent;
                    child.set_right_child(Some(node.take()));
                    Node::make_child_of(child, parent);
                } else {
                    child.set_right_child(Some(node.take()));
                    Node::make_root(child);
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
