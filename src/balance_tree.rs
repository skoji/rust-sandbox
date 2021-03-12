pub type Tree = Option<Box<Node>>;

pub struct Node {
    pub number: i32,
    left: Tree,
    right: Tree,
}

pub struct Btree {
    pub root: Tree,
    pub size: u64,
}

impl Btree {
    pub fn new() -> Self {
        Btree {
            root: None,
            size: 0,
        }
    }

    fn add_internal(node: Tree, value: i32) -> Box<Node> {
        match node {
            Some(mut t) => {
                if t.number <= value {
                    t.left = Some(Btree::add_internal(t.left, value));
                    t
                } else {
                    t.right = Some(Btree::add_internal(t.right, value));
                    t
                }
            }
            None => Box::new(Node {
                number: value,
                left: None,
                right: None,
            }),
        }
    }

    pub fn add(&mut self, value: i32) {
        self.size += 1;
        let root = self.root.take();
        self.root = Some(Btree::add_internal(root, value));
    }
}

impl Default for Btree {
    fn default() -> Self {
        Btree::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_new_tree() {
        let tree = Btree::new();
        assert_eq!(tree.size, 0);
    }

    #[test]
    fn add_values() {
        let mut tree = Btree::new();
        tree.add(12);
        tree.add(0);
        tree.add(19);

        let root = &tree.root;
        match root {
            Some(ref s) => {
                assert_eq!(s.number, 12);
                assert_eq!(s.left.as_ref().unwrap().number, 19);
                assert_eq!(s.right.as_ref().unwrap().number, 0);
            }
            None => {
                unreachable!()
            }
        }
    }
}
