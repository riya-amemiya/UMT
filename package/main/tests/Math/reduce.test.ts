import { reduce } from '../../module/Math/reduce';
test('{reduce}', () => {
    expect(reduce(16, 2)).toBe({
        x: 8,
        y: 1,
        gcd: 2,
    });
});
