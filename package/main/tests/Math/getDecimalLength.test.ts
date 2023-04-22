import { getDecimalLength } from '../../module/Math/getDecimalLength';
test('{getDecimalLength}', () => {
    expect(getDecimalLength(1)).toBe(0);
    expect(getDecimalLength(1.1)).toBe(1);
    expect(getDecimalLength(1.11)).toBe(2);
});
