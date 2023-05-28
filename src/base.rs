use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct RawNode<T> where T: PartialEq {
    value: T,
    children: RefCell<Vec<Rc<RawNode<T>>>>,
    pub parent: RefCell<Weak<RawNode<T>>>,
}

#[allow(unused)]
impl<T> RawNode<T> where T: PartialEq {
    pub fn new(value: T) -> Self {
        Self { value, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) }
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    pub fn parent(&self) -> Option<Rc<Self>> {
        self.parent.borrow().upgrade()
    }

    pub const fn children(&self) -> &RefCell<Vec<Rc<Self>>> {
        &self.children
    }

    pub fn add_child(self: &Rc<Self>, child: &Rc<Self>) -> bool {
        return if self.get_root().unique_nodes(child) {
            self.children.borrow_mut().push(Rc::clone(child));
            *child.parent.borrow_mut() = Rc::downgrade(self);
            true
        } else {
            false
        };
    }

    pub fn find(self: &Rc<Self>, value: &T) -> Option<Rc<Self>> {
        if self.value() == value {
            Some(self.clone())
        } else {
            self.children.borrow().iter().find_map(|node| Self::find(node, value))
        }
    }

    pub fn remove_node(self: &Rc<Self>, value: &T) -> Option<Rc<Self>> {
        if let Some(node) = self.find(value) {
            if let Some(parent) = node.parent.borrow().upgrade() {
                let mut vec = parent.children.borrow_mut();
                let idx = vec.iter().position(|node| node.value() == value).unwrap();
                return Some(vec.remove(idx));
            }
        }
        None
    }

    pub fn get_root(self: &Rc<Self>) -> Rc<Self> {
        return if self.parent().is_none() {
            self.clone()
        } else {
            Self::get_root(&self.parent.borrow().upgrade().unwrap())
        };
    }

    fn as_array(self: &Rc<Self>, mut elements: &mut Vec<Rc<Self>>, parent: &Rc<Self>) {
        if self.parent().is_none() || (self.parent().is_some() && &self.parent().unwrap() == parent) {
            elements.push(self.clone());
        }

        for child in self.children.borrow().iter() {
            elements.push(child.clone());
        }
        self.children.borrow().iter().for_each(|child| Self::as_array(child, elements, parent));
    }

    pub fn unique_nodes(self: &Rc<Self>, other: &Rc<Self>) -> bool {
        let mut my_nodes = vec![];
        let mut other_nodes = vec![];
        self.as_array(&mut my_nodes, self);
        other.as_array(&mut other_nodes, self);
        let my_values: Vec<&T> = my_nodes.iter().map(|n| n.value()).collect();
        let other_values: Vec<&T> = other_nodes.iter().map(|n| n.value()).collect();

        !other_values.iter().any(|o| my_values.contains(o))
    }
}

impl<T> PartialEq for RawNode<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}