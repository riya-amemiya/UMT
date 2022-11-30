import { softmax } from '../../module/Math/softmax';
test('{softmax}', () => {
    expect(softmax([2, 1, 0])).toEqual(
        expect.arrayContaining([0.665, 0.245, 0.09]),
    );
});
