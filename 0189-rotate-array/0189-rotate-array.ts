/**
 Do not return anything, modify nums in-place instead.
 */
function rotate(nums: number[], k: number): void {
    nums.unshift(...nums.splice(nums.length - (k % nums.length)));
};