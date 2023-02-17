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

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev = None;
        let mut min = i32::MAX;
        Self::inorder(root, &mut prev, &mut min);
        min   
    }
    
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min:&mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder(node.left.clone(), prev, min);
            if let Some(prev) = prev {
                *min = (*min).min(node.val - *prev);
            }
            *prev = Some(node.val);
            Self::inorder(node.right.clone(), prev, min);
    }    
}
}