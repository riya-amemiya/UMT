import { primeFactorization } from '../../module/Math/primeFactorization';
test('{primeFactorization}', () => {
    expect(primeFactorization(1)).toBe([
        {
            name: '1',
            count: 1,
        },
    ]);
    expect(primeFactorization(49)).toBe([
        {
            name: '7',
            count: 2,
        },
    ]);
});
