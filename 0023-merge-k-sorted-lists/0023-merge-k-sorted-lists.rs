impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lst = std::collections::BinaryHeap::new();
        let mut ans = None;

        for mut list in lists {
            while let Some(node) = list {
                lst.push(node.val);
                list = node.next;
            }
        }

        while let Some(val) = lst.pop() {
            ans = Some(Box::new(ListNode { val, next: ans }));
        }
        ans
    }
}