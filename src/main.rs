use tree::Tree;

mod tree;

fn main() {
    let mut tree = Tree::new(80);
    tree.insert(70);
    tree.insert(50);
    tree.insert(40);
    tree.insert(60);
    tree.insert(30);
    tree.insert(89);
    tree.print();
}