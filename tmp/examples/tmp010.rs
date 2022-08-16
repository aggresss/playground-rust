// Reference: https://blog.csdn.net/linysuccess/article/details/124939459

use std::{
    cell::RefCell,
    rc::Rc, vec,
};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
}

pub fn inorder_traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        if let Some(x) = root {
            dfs(x.borrow().left.clone(), list);
            list.push(x.borrow().val);
            dfs(x.borrow().right.clone(), list);
        }
    }
    let mut list: Vec<i32> = vec![];
    dfs(root, &mut list);
    list
}

fn main() {
    let mut s1: TreeNode = TreeNode::new(2);
    let s2: TreeNode = TreeNode::new(1);
    let s3: TreeNode = TreeNode::new(3);
    s1.left = Some(Rc::new(RefCell::new(s2)));
    s1.right = Some(Rc::new(RefCell::new(s3)));
    let root = Some(Rc::new(RefCell::new(s1)));
    println!("{:?}", inorder_traverse(root));
}
