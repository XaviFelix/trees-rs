use crate::tree_node::{TreeLink, TreeNode};
use std::cmp::max;

// (Root → Left → Right)
pub fn preorder<T: Clone>(root: &TreeLink<T>, output: &mut Vec<T>) {
    if let Some(rc) = root {
        let node = rc.borrow();
        output.push(node.val.clone());
        preorder(&node.left, output);
        preorder(&node.right, output);
    }
}

// (Left → Root → Right)
pub fn inorder<T: Clone>(root: &TreeLink<T>, output: &mut Vec<T>) {
    if let Some(rc) = root {
        let node = rc.borrow();
        inorder(&node.left, output);
        output.push(node.val.clone());
        inorder(&node.right, output);
    }
}

// (Left → Right → Root)
pub fn postorder<T: Clone>(root: &TreeLink<T>, output: &mut Vec<T>) {
    if let Some(rc) = root {
        let node = rc.borrow();
        postorder(&node.left, output);
        postorder(&node.right, output);
        output.push(node.val.clone());
    }
}

pub fn max_depth<T: Clone>(root: &TreeLink<T>) -> usize {
    match root {
        Some(rc) => {
            let node = rc.borrow();
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + max(left_depth, right_depth)
        }
        None => 0,
    }
}

pub fn build_tree<T: Clone>(values: &[T]) -> TreeLink<T> {
    if values.is_empty() {
        return None;
    }

    let nodes: Vec<TreeLink<T>> = values.iter().cloned().map(TreeNode::new).collect();

    for i in 0..values.len() {
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < values.len() {
            TreeNode::set_left(&nodes[i], nodes[left].clone());
        }

        if right < values.len() {
            TreeNode::set_right(&nodes[i], nodes[right].clone());
        }
    }

    nodes[0].clone()
}
