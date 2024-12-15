use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T: Eq + PartialOrd + Clone> {
    pub left: Link<T>,
    pub right: Link<T>,
    pub value: T,
}

impl<T: Eq + PartialOrd + Clone> Clone for Node<T> {
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }

    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<T: Eq + PartialOrd + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }
}

pub struct Tree<T: Eq + PartialOrd + Clone> {
    pub root: Link<T>,
    pub lenght: u16,
}

impl<T: Eq + PartialOrd + Clone> Tree<T> {
    pub fn new(value: T) -> Self {
        let n = Node::new(value);
        Self {
            root: Some(Box::new(n)),
            lenght: 1,
        }
    }

    pub fn insert(&mut self, value: T) {
        let n = Some(Box::new(Node::new(value.clone())));
        let root: &mut Box<Node<T>> = self.root.as_mut().unwrap();
        if value > root.value && root.as_ref().right.is_none() {
            root.right = n;
            self.lenght += 1;
            return;
        }

        if value < root.value && root.as_ref().left.is_none() {
            root.left = n;
            self.lenght += 1;
            return;
        }

        self.insert_help(value);
    }

    fn insert_help(&mut self, value: T) {
        let mut curr: &mut Box<Node<T>> = self.root.as_mut().unwrap();
        let n = Some(Box::new(Node::new(value.clone())));

        while curr.left.is_some() || curr.right.is_some() {
            if value == curr.value {
                return;
            }
            if value > curr.value {
                if curr.right.is_some() {
                    curr = curr.right.as_mut().unwrap();
                    continue;
                }
                break;
            }

            if value < curr.value {
                if curr.left.is_some() {
                    curr = curr.left.as_mut().unwrap();
                    continue;
                }
                break;
            }
        }
        if value > curr.value {
            curr.right = n.clone()
        }

        if value < curr.value {
            curr.left = n
        }
    }
}

impl<T: Eq + PartialOrd + Clone + Debug> Tree<T> {
    pub fn print(&self) {
        self.print_node(&self.root, 0);
    }

    fn print_node(&self, node: &Option<Box<Node<T>>>, depth: usize) {
        if let Some(node) = node {
            // Print right subtree
            self.print_node(&node.right, depth + 1);

            // Indent according to depth
            for _ in 0..depth {
                print!("   ");
            }
            println!("{:?}", node.value);

            // Print left subtree
            self.print_node(&node.left, depth + 1);
        }
    }
}
