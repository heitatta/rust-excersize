use std::cmp::{Ord, Ordering};
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, v: T) -> bool{
        let branch = match v.cmp(&self.value) {
            Ordering::Less => &mut self.left,
            Ordering::Greater => &mut self.right,
            Ordering::Equal => return false,
        };
        match branch {
            Some(node) => node.insert(v),
            None => {
                *branch = Some(Box::new(Node::new(v)));
                true
            }
        }
    }

    pub fn contains(&self, v: &T) -> bool {
        match v.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => self.left.as_ref().map_or(false, |node| node.contains(v)),
            Ordering::Greater => self.right.as_ref().map_or(false, |node| node.contains(v)),
        }
    }

    pub fn vec(&self) -> Vec<&T> {
        let mut result = Vec::new();
        if let Some(left) = &self.left {
            result.extend(left.vec());
        }
        result.push(&self.value);
        if let Some(right) = &self.right {
            result.extend(right.vec());
        }
        result
    }
}

impl<T> Display for Node<T> where T: Ord + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", format!("{:?}", self.vec()))
    }
}
