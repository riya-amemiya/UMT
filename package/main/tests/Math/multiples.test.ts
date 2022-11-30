import { multiples } from '../../module/Math/multiples';
test('{multiples}', () => {
    expect(multiples(1, 1)).toBe(1);
    expect(multiples(1, 2)).toBe(2);
    expect(multiples(1, 3)).toBe(3);
    expect(multiples(1.5, 2)).toBe(3);
});
