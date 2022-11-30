import { softmax } from '../../module/Math/softmax';
test('{softmax}', () => {
    expect(softmax([1, 2, 3])).toStrictEqual([
        0.09003057317038046, 0.24472847105479764, 0.6652409557748219,
    ]);
});
