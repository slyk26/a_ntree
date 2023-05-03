#![allow(rustdoc::invalid_codeblock_attributes)]
#![doc = include_str!("../README.md")]

use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::Debug;

#[derive(Debug)]
/// a singular Node that holds a generic value
pub struct Node<T> where T: PartialEq {
    pointer: Rc<RawNode<T>>,
}

#[allow(unused)]
impl<T> Node<T> where T: PartialEq {
    /// creates a new [Node] with a value
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let node = Node::new(10);
    /// let another_node = Node::new(10);
    ///
    /// assert_ne!(node, another_node);
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
    /// if the Node is the root, the Parent is [None]
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
    /// assert_eq!(root.children().get(0).unwrap(), &child);
    /// assert_eq!(child.parent().unwrap(), root);
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
    /// same as [Node::add_child()] but without the need to create a new Node
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

    /// searches a [Node] by value - starting from the calling Node inclusive
    ///
    /// returns the first Node found or [None] if the value doesnt exist
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

    /// removes the first child [Node] from this Node
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    ///
    /// let root = Node::new(10);
    /// root.add_leaf(30);
    /// root.add_leaf(40);
    /// // root has 2 children
    /// root.remove_node(&40);
    /// // root has 1 child
    /// assert_eq!(root.children().len(), 1);
    /// assert!(root.find(&40).is_none());
    /// ```
    pub fn remove_node(&self, value: &T) {
        self.pointer.remove_node(value);
    }

    /// get the root [Node]
    ///
    /// if this Node has no parents, this Node is the root Node
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// let a_child = Node::new(20);
    /// let child_of_child = Node::new(30);
    /// root.add_child(&a_child);
    /// a_child.add_child(&child_of_child);
    ///
    /// assert_eq!(child_of_child.get_root(), root);
    /// ```
    pub fn get_root(&self) -> Node<T> {
        Node::from(&self.pointer.get_root())
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
            self.children.borrow().iter().find_map(|node| RawNode::find(node, value))
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

    fn get_root(self: &Rc<Self>) -> Rc<RawNode<T>> {
        let mut ret;
        if self.parent().is_none() {
            ret = self.clone()
        } else {
            ret = RawNode::get_root(&self.parent.borrow().upgrade().unwrap());
        }
        ret
    }
}