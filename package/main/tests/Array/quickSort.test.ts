import { quickSort } from '../../module/Array/quickSort';
test('{quickSort}', () => {
    expect(quickSort([1, 2, 3])).toEqual([1, 2, 3]);
    expect(quickSort([3, 2, 1])).toEqual([1, 2, 3]);
    expect(quickSort([1, 3, 2])).toEqual([1, 2, 3]);
});
