function mergeArrays(nums1: number[][], nums2: number[][]): number[][] {
    const map: Map<number, number> = new Map();

    for (const [id, val] of nums1) {
        map.set(id, val);
    }

    for (const [id, val] of nums2) {
        map.set(id, (map.get(id) || 0) + val);
    }

    const result: number[][] = [];
    for (const [id, val] of map) {
        result.push([id, val]);
    }

    return result.sort((a, b) => a[0] - b[0]);
}