//! This crate implements a simple generic Tree datatype with Nodes.
//! # Limitations
//! This datatype has the following limitations:
//! * Each Value inside a [Node] is unique
//! * A [Node] cannot have children of different types
//! # Example
//! ```
//!  use a_ntree::Node;
//!  let root = Node::new(10);
//!  let a_child = Node::new(20);
//!
//!  root.add_child(&a_child);
//!  a_child.add_leaf(999);
//!  root.add_leaf(30);
//!
//!     //        Node(10)
//!     //        /     \
//!     //    Node(20) Node(30)
//!     //      /
//!     //  Node(999)
//! ```

use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Debug)]
/// a singular Node that holds a generic value
pub struct Node<T> where T: PartialEq {
    pointer: Rc<RawNode<T>>,
}

impl<T> PartialEq for Node<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.pointer, &other.pointer)
    }
}

#[allow(unused)]
impl<T> Node<T> where T: PartialEq {
    /// creates a new [Node] with a value
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let node = Node::new(10);
    /// let another_node = Node::new("Burger");
    /// ```
    pub fn new(value: T) -> Self {
        Node { pointer: Rc::new(RawNode::new(value)) }
    }

    /// internal method to get a [RawNode] as a [Node]
    fn from(pointer: &Rc<RawNode<T>>) -> Self {
        Node { pointer: Rc::clone(pointer) }
    }

    /// returns the value of a [Node] by reference
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let node = Node::new(10);
    ///
    /// assert_eq!(node.value(), &10)
    /// ```
    pub fn value(&self) -> &T {
        self.pointer.value()
    }

    /// returns the parent of a [Node],
    ///
    /// if the [Node] is the root, the Parent is [None]
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// let child = Node::new(20);
    /// root.add_child(&child);
    ///
    /// assert!(root.parent().is_none());
    /// assert_eq!(child.parent().unwrap(), root);
    /// ```
    pub fn parent(&self) -> Option<Node<T>> {
        if let Some(parent) = &self.pointer.parent.borrow().upgrade() {
            return Some(Node::from(parent));
        }
        None
    }

    /// returns the children of a [Node]
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// let child = Node::new(30);
    /// root.add_child(&child);
    ///
    /// assert_eq!(root.children().get(0).unwrap(), &child)
    pub fn children(&self) -> Vec<Node<T>> {
        let mut ret: Vec<Node<T>> = vec![];

        self.pointer.children.borrow().iter().for_each(|child| {
            ret.push(Node::from(child));
        });

        ret
    }

    /// adds a child to a [Node]
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// let child = Node::new(20);
    /// root.add_child(&child);
    ///
    /// assert_eq!(child.parent().unwrap(), root);
    /// ```
    pub fn add_child(&self, child: &Node<T>) {
        self.pointer.add_child(&child.pointer);
    }

    /// adds a value directly as a child to a [Node]
    ///
    /// same as [Node::add_child()] but without the need to create a new [Node]
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// root.add_leaf(30);
    ///
    /// assert_eq!(root.children().get(0).unwrap().value(), &30);
    /// ```
    pub fn add_leaf(&self, leaf: T) {
        self.add_child(&Node::new(leaf));
    }

    /// searches a [Node] by value - starting from the calling [Node] inclusive
    ///
    /// returns [None] if Node doesnt exist
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// root.add_leaf(20);
    /// root.add_leaf(30);
    ///
    /// assert_eq!(root.find(&30).unwrap().value(), &30);
    /// assert_eq!(root.find(&20).unwrap().value(), &20);
    /// assert_eq!(root.find(&10).unwrap().value(), &10);
    /// assert!(root.find(&999999).is_none());
    ///```
    pub fn find(&self, value: &T) -> Option<Node<T>> {
        if let Some(found) = self.pointer.find(&value) {
            return Some(Node::from(&found));
        }
        None
    }

    pub fn remove_node(&self, value: &T) {
        self.pointer.remove_node(value);
    }
}

#[derive(Debug)]
struct RawNode<T> where T: PartialEq {
    value: T,
    parent: RefCell<Weak<RawNode<T>>>,
    children: RefCell<Vec<Rc<RawNode<T>>>>,
}

#[allow(unused)]
impl<T> RawNode<T> where T: PartialEq {
    fn new(value: T) -> Self {
        RawNode { value, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) }
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn parent(&self) -> Option<Rc<RawNode<T>>> {
        self.parent.borrow().upgrade()
    }

    fn children(&self) -> &RefCell<Vec<Rc<RawNode<T>>>> {
        &self.children
    }

    fn add_child(self: &Rc<Self>, child: &Rc<RawNode<T>>) {
        self.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(self);
    }

    fn find(self: &Rc<Self>, value: &T) -> Option<Rc<RawNode<T>>> {
        if self.value() == value {
            Some(self.clone())
        } else {
            let mut ret = None;
            for i in 0..self.children.borrow().len() {
                ret = RawNode::find(self.children().borrow().get(i).unwrap(), value);
            }
            ret
        }
    }

    fn remove_node(self: &Rc<Self>, value: &T) {
        if let Some(node) = self.find(value) {
            if let Some(parent) = node.parent.borrow().upgrade() {
                let mut vec = parent.children.borrow_mut();
                let idx = vec.iter().position(|node| node.value() == value).unwrap();
                vec.remove(idx);
            }
        }
    }
}