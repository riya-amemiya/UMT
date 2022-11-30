import { quotient } from '../../module/Math/quotient';
test('{quotient}', () => {
    expect(quotient(1, 1)).toEqual(expect.arrayContaining([1, 0]));
    expect(quotient(1, 2)).toEqual(expect.arrayContaining([0, 1]));
    expect(quotient(2, 1)).toEqual(expect.arrayContaining([2, 0]));
});
