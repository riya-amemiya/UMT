import { factorize } from '../../module/Math/factorize';
test('{factorize}', () => {
    expect(factorize(1)).toBe(1);
    expect(factorize(2)).toBe(2);
});
