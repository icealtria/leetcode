function pivotArray(nums: number[], pivot: number): number[] {
    const lessThan: number[] = [];
    const equals: number[] = [];
    const greaterThan: number[] = [];

    for (const num of nums) {
        if (num < pivot) {
            lessThan.push(num);
        } else if (num === pivot) {
            equals.push(num);
        } else {
            greaterThan.push(num);
        }
    }

    return [...lessThan, ...equals, ...greaterThan];
}