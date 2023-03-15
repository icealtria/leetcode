// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut q = VecDeque::from([root]);
        let mut seen_null = false;
        while let Some(option) = q.pop_front() {
            if let Some(node) = option {
                if seen_null {
                    return false;
                }
                let node = node.borrow();
                q.push_back(node.left.clone());
                q.push_back(node.right.clone());
            } else {
                seen_null = true;
            }
        }
        true
    }
}
