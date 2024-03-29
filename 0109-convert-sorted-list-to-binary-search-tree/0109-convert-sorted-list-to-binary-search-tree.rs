// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = head;
        let mut nums = Vec::new();
        while let Some(node) = head {
            nums.push(node.val);
            head = node.next;
        }
        helper(&nums)
    }
}

fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = nums.len();

    match n {
        0 => None,
        _ => {
            let m = n / 2;
            let mut node = TreeNode::new(nums[m]);
            node.left = helper(&nums[0..m]);
            node.right = helper(&nums[m + 1..]);

            Some(Rc::new(RefCell::new(node)))
        }
    }
}
