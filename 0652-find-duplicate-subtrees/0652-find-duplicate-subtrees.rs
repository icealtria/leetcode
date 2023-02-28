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
use std::collections::HashMap;
use std::cell::RefCell;
impl Solution {
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
            map: &mut HashMap<String, i32>,
        ) -> String {
            match root {
                Some(node) => {
                    let left = dfs(node.borrow().left.clone(), res, map);
                    let right = dfs(node.borrow().right.clone(), res, map);
                    let s = format!("{} {} {}", node.borrow().val, left, right);
                    if let Some(&v) = map.get(&s) {
                        if v == 1 {
                            res.push(Some(node));
                        }
                    }
                    *map.entry(s.clone()).or_insert(0) += 1;
                    s
                }
                None => "".to_string(),
            }
        }
        
        let mut map = HashMap::new();
        let mut res = Vec::new();
        dfs(root, &mut res, &mut map);
        res
    }
}