#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

mod base;
use std::rc::Rc;
use std::fmt::Debug;
use crate::base::RawNode;

#[derive(Debug)]
/// a singular Node that holds a generic value
pub struct Node<T> where T: PartialEq {
    pointer: Rc<RawNode<T>>,
}

impl<T> PartialEq for Node<T> where T: PartialEq {
    #[inline]
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
    /// let another_node = Node::new(10);
    ///
    /// assert_ne!(node, another_node);
    /// ```
    pub fn new(value: T) -> Self {
        Self { pointer: Rc::new(RawNode::new(value)) }
    }

    /// internal method to get a [`RawNode`] as a [`Node`]
    fn from(pointer: &Rc<RawNode<T>>) -> Self {
        Self { pointer: Rc::clone(pointer) }
    }

    /// returns the value of a [Node] by reference
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let node = Node::new(10);
    ///
    /// assert_eq!(node.value(), &10)
    /// ```
    #[must_use]
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
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        if let Some(parent) = &self.pointer.parent.borrow().upgrade() {
            return Some(Self::from(parent));
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
    #[must_use]
    pub fn children(&self) -> Vec<Self> {
        let mut ret: Vec<Self> = vec![];

        self.pointer.children().borrow().iter().for_each(|child| {
            ret.push(Self::from(child));
        });

        ret
    }

    /// adds a child to a [Node] if the child or any of its children are not in the tree
    ///
    /// returns true if it added, else false
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// let child = Node::new(20);
    /// let should_be_true = root.add_child(&child);
    /// let should_be_false = root.add_child(&child);
    ///
    /// assert_eq!(should_be_true, true);
    /// assert_eq!(should_be_false, false);
    /// ```
    #[must_use]
    pub fn add_child(&self, child: &Self) -> bool {
        self.pointer.add_child(&child.pointer)
    }

    /// adds a value directly as a child to a [`Node`]
    ///
    /// same as [`Node::add_child()`] but without the need to create a new Node
    /// ## Example
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// root.add_leaf(30);
    ///
    /// assert_eq!(root.children().get(0).unwrap().value(), &30);
    /// ```
    pub fn add_leaf(&self, leaf: T) -> bool {
        self.add_child(&Self::new(leaf))
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
    pub fn find(&self, value: &T) -> Option<Self> {
        if let Some(found) = self.pointer.find(value) {
            return Some(Self::from(&found));
        }
        None
    }

    /// removes the first child [Node] from this Node and all children
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
    pub fn remove_node(&self, value: &T) -> Option<Self> {
        self.pointer.remove_node(value).map(|raw_node| Self::from(&raw_node))
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
    #[must_use]
    pub fn get_root(&self) -> Self {
        Self::from(&self.pointer.get_root())
    }

    /// gets the number of strong pointers of this Node
    /// ```
    /// use a_ntree::Node;
    /// let root = Node::new(10);
    /// assert_eq!(root.rc_count(), 1);
    /// ```
    #[must_use]
    pub fn rc_count(&self) -> usize  {
     Rc::strong_count(&self.pointer)
    }
}
