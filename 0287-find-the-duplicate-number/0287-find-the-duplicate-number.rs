impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let head = nums[0] as i32;
        let next = |i| nums[i as usize] as i32;
        let (mut slow, mut fast) = (head, head);
        loop {
            slow = next(slow);
            fast = next(next(fast));
            if slow == fast {
                break;
            }
        }

        slow = nums[0];
        while slow != fast {
            slow = next(slow);
            fast = next(fast);
        }

        return slow;
    }
}