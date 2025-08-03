use tree_link::tree_ops::*;

fn main() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let root = build_tree(&values);

    let mut out = Vec::new();
    inorder(&root, &mut out);
    println!("in-order: {:?}", out);
    println!("max depth: {:?}", max_depth(&root));
}
