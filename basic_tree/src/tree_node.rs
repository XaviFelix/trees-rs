use std::cell::RefCell;
use std::rc::Rc;

pub type TreeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: TreeLink<T>,
    pub right: TreeLink<T>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> TreeLink<T> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }

    pub fn set_left(node: &TreeLink<T>, child: TreeLink<T>) {
        if let Some(rc) = node {
            rc.borrow_mut().left = child;
        }
    }

    pub fn set_right(node: &TreeLink<T>, child: TreeLink<T>) {
        if let Some(rc) = node {
            rc.borrow_mut().right = child;
        }
    }
}
