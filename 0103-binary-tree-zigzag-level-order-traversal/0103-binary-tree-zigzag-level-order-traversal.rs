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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }

        let mut queue = VecDeque::new();
        let mut zig = false;
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut level = Vec::new();
            let len = queue.len();
            for _ in 0..len {
                if let Some(node) = queue.pop_front() {
                    level.push(node.borrow().val);
                    
                    if let Some(left) = &node.borrow().left {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node.borrow().right {
                        queue.push_back(Rc::clone(right));
                    }
                }
            }
            if zig {
                level.reverse();
            }
            zig = !zig;
            res.push(level);
        }
        res
    }
}