import { valueSwap } from '../../module/Math/valueSwap';
test('{valueSwap}', () => {
    expect(valueSwap(1, 2)).toBe([2, 1]);
});
