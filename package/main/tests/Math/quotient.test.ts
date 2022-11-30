import { quotient } from '../../module/Math/quotient';
test('{quotient}', () => {
    expect(quotient(1, 1)).toBe(1);
    expect(quotient(2, 1)).toBe(2);
    expect(quotient(2, 2)).toBe(1);
});
